use std::future::{ready, Ready};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::LocalBoxFuture;
use the_logger::{log_trace, TheLogger};

pub struct ApiLogger;

impl<S, B> Transform<S, ServiceRequest> for ApiLogger
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware { service }))
    }
}

pub struct LoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let method = req.method().to_string();
        let path = req.path().to_string();
        tokio::spawn({
            api_log_trace_request(
                method,
                path
            )
        });

        let fut = self.service.call(req);

        Box::pin(async move {

            let res: ServiceResponse<B> = fut.await?;

            let status_code = res.status().to_string();

            tokio::spawn(api_log_trace_response(status_code));

            Ok(res)
        })
    }
}

async fn api_log_trace_request(method: String, path: String) {
    let logger = TheLogger::instance();
    log_trace!(logger, "Request: {} {}", method, path);
}

async fn api_log_trace_response(status_code: String) {
    let logger = TheLogger::instance();
    log_trace!(logger, "Response: {}", status_code);
}
