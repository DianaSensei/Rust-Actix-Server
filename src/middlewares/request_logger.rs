use actix_web::body::MessageBody;
use actix_web::dev::{Service, Transform};
use actix_web::http::header;
use actix_web::http::Method;
use actix_web::web::BytesMut;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use futures::future::{ok, Future, Ready};
use futures::StreamExt;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct LoggingRequestMiddleware;

impl<S: 'static, B> Transform<S, ServiceRequest> for LoggingRequestMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
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

#[allow(clippy::type_complexity)]
#[allow(clippy::needless_question_mark)]
impl<S: 'static, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let req_start = chrono::Local::now();
        let req_uuid = uuid::Uuid::new_v4()
            .as_simple()
            .encode_lower(&mut uuid::Uuid::encode_buffer())
            .to_string();
        info!("----- Start Request [{}]", req_uuid);

        info!(
            "{:#?}: {:#?}",
            header::USER_AGENT.as_str(),
            req.headers()
                .get(header::USER_AGENT)
                .unwrap_or(&header::HeaderValue::from_str("").unwrap())
        );

        let s = if let Some(peer) = req.connection_info().peer_addr() {
            (*peer).to_string()
        } else {
            "-".to_string()
        };
        info!("{:#?}: {:#?}", header::FROM.as_str(), s);

        if req.headers().get(header::AUTHORIZATION).is_some() {
            info!(
                "{:#?}: \"****************************\"",
                header::AUTHORIZATION.as_str()
            );
        }

        info!("\"path\": {:#?} {:#?}", req.method(), req.path());
        if !req.query_string().is_empty() {
            info!("\"query\": {:#?}", req.query_string());
        }

        let svc = self.service.clone();

        Box::pin(async move {
            let mut body = BytesMut::new();
            let mut stream = req.take_payload();
            while let Some(chunk) = stream.next().await {
                body.extend_from_slice(&chunk?);
            }

            if !body.size().is_eof()
                && !(req.method() == Method::GET)
                && !(req.method() == Method::HEAD)
            {
                let json: serde_json::Value = serde_json::from_slice(&*body).unwrap();
                info!("\"body\": {}", json);
            }

            let (_, mut payload) = actix_http::h1::Payload::create(true);
            payload.unread_data(body.freeze());
            req.set_payload(payload.into());

            let res = svc.call(req).await?;
            info!(
                "------- End Request [{}] in `{}ms`",
                req_uuid,
                (chrono::Local::now() - req_start).num_milliseconds()
            );
            // Wait for next process
            Ok(res)
        })
    }
}
