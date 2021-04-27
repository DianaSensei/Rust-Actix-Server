use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::body::{BodySize, MessageBody, ResponseBody};
use actix_web::web::{Bytes, BytesMut};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use pin_project::__private::PinnedDrop;
use futures::StreamExt;

pub struct PosRequest;

impl<S: 'static, B> Transform<S, ServiceRequest> for PosRequest
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BodyLogger<B>>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware { service })
    }
}

pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<BodyLogger<B>>;
    type Error = Error;
    type Future = WrapperStream<S, B, ServiceRequest>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        WrapperStream {
            fut: self.service.call(req),
            _t: PhantomData,
        }
    }
}

#[pin_project::pin_project]
pub struct WrapperStream<S, B, Req>
where
    B: MessageBody,
    S: Service<Req>,
{
    #[pin]
    fut: S::Future,
    _t: PhantomData<(B,)>,
}

impl<S, B, Req> Future for WrapperStream<S, B, Req>
where
    B: MessageBody,
    S: Service<Req, Response = ServiceResponse<B>, Error = Error>,
{
    type Output = Result<ServiceResponse<BodyLogger<B>>, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = futures::ready!(self.project().fut.poll(cx));
        Poll::Ready(res.map(|res| {
            res.map_body(move |_, body| {
                // info!("Response {:?}", body.tak);
                ResponseBody::Body(BodyLogger {
                    body,
                    body_accum: BytesMut::new(),
                })
            })
        }))
    }
}

#[pin_project::pin_project(PinnedDrop)]
pub struct BodyLogger<B> {
    #[pin]
    body: ResponseBody<B>,
    body_accum: BytesMut,
}

#[pin_project::pinned_drop]
impl<B> PinnedDrop for BodyLogger<B> {
    fn drop(self: Pin<&mut Self>) {
        info!("drop");
        // let data = self.
        // let json: serde_json::Value = serde_json::from_slice(&*bytes).unwrap();
        // info!("Response body: {:#?}", json);
    }
}

impl<B: MessageBody> MessageBody for BodyLogger<B> {
    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, Error>>> {
        let this = self.project();
        match this.body.poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => {
                Poll::Ready(Some(Ok(chunk)))
            }
            Poll::Ready(Some(Err(e))) => {
                Poll::Ready(Some(Err(e)))
            },
            Poll::Ready(None) => {
                Poll::Ready(None)
            },
            Poll::Pending => {
                Poll::Pending
            },
        }
    }
}
