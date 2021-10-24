use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::dev::{MessageBody, Service, Transform};
use actix_web::http::Method;
use actix_web::http::header;
use futures::future::{ok, Future, Ready};
use actix_web::web::BytesMut;
use futures::StreamExt;

pub struct LoggingRequestMiddleware;

impl<S: 'static, B> Transform<S> for LoggingRequestMiddleware
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for LoggingMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        info!("");
        info!("{:#?}: {:#?}", header::ACCEPT.as_str(), req.headers().get(header::ACCEPT).unwrap_or(&header::HeaderValue::from_str("").unwrap()));
        info!("{:#?}: {:#?}", header::USER_AGENT.as_str(), req.headers().get(header::USER_AGENT).unwrap_or(&header::HeaderValue::from_str("").unwrap()));
        info!("{:#?}: {:#?}", header::HOST.as_str(), req.headers().get(header::HOST).unwrap_or(&header::HeaderValue::from_str("").unwrap()));
        if let Some(_) = req.headers().get(header::AUTHORIZATION) {
            info!("{:#?}: \"****************************\"", header::AUTHORIZATION.as_str());
        }

        info!("\"path\": {:#?} {:#?}", req.method(), req.path());
        if !req.query_string().is_empty() {
            info!("\"query\": {:#?}", req.query_string());
        }

        let mut svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            if !body.size().is_eof()
                && !(req.method() == Method::GET)
                && !(req.method() == Method::HEAD) {
                let json: serde_json::Value = serde_json::from_slice(&*body).unwrap();
                // let request = ServiceRequest::from(req);
                info!("\"body\": {}", json);
            }

            // Wait for next process
            Ok(svc.call(req).await?)
        })
    }
}
