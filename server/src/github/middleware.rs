use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct VerifyGithubSignatureFactory;

// Middleware factory is `Transform` trait
impl<NextService, ResponseBody> Transform<NextService, ServiceRequest>
    for VerifyGithubSignatureFactory
where
    NextService: Service<ServiceRequest, Response = ServiceResponse<ResponseBody>, Error = Error>,
    NextService::Future: 'static,
    ResponseBody: 'static,
{
    type Response = ServiceResponse<ResponseBody>;
    type Error = Error;
    type InitError = ();
    type Transform = VerifyGithubSignatureMiddleware<NextService>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: NextService) -> Self::Future {
        ready(Ok(VerifyGithubSignatureMiddleware { service }))
    }
}

pub struct VerifyGithubSignatureMiddleware<NextService> {
    service: NextService,
}

impl<NextService, ResponseBody> Service<ServiceRequest>
    for VerifyGithubSignatureMiddleware<NextService>
where
    NextService: Service<ServiceRequest, Response = ServiceResponse<ResponseBody>, Error = Error>,
    NextService::Future: 'static,
    ResponseBody: 'static,
{
    type Response = ServiceResponse<ResponseBody>;
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
