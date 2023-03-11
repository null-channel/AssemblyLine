use std::future::{ready, Ready};


use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform, Payload},
    Error,
};
use futures_util::future::LocalBoxFuture;

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

    /*
    But actually, the logic should look maybe something like this? 
    use std::io::Read;

        let r = req.clone();
        let s = r.headers()
            .get("X-Hub-Signature")
            .ok_or(ErrorUnauthorized(ParseError::Header))?
            .to_str()
            .map_err(ErrorUnauthorized)?;
        // strip "sha1=" from the header
        let (_, sig) = s.split_at(5);

        let secret = env::var("GITHUB_SECRET").unwrap();
        let mut body = String::new();
        req.read_to_string(&mut body)
            .map_err(ErrorInternalServerError)?;

        if is_valid_signature(&sig, &body, &secret) {
            Ok(Started::Done)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
         */

    // This is the actual middleware logic
    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Hi from response");
            Ok(res)
        })
    }
}