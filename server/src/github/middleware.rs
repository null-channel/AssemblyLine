use std::future::{ready, Ready};


use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform, Payload},
    Error, web::BytesMut,
};
use futures_util::future::LocalBoxFuture;
use urlencoding::decode;
use futures::StreamExt;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct VerifyGithubSignatureFactory;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for VerifyGithubSignatureFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyGithubSignatureMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(VerifyGithubSignatureMiddleware { service }))
    }
}

pub struct VerifyGithubSignatureMiddleware<S> {
    service: S,
}
const MAX_SIZE: usize = 262_144;
impl<S, B> Service<ServiceRequest> for VerifyGithubSignatureMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // This is the actual middleware logic
    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // Validate the signature here

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}