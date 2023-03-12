use std::env;

use actix_web::{dev::{Service as _, Server}, web::{self,Bytes, JsonConfig}, App,post, HttpServer, Responder, error, HttpResponse, Result};
use futures_util::future::FutureExt;
use actix_web::error::{ErrorUnauthorized, ParseError};
use crate::github::middleware::VerifyGithubSignatureFactory;
use futures::StreamExt;
use urlencoding::decode;

use super::data::webhook_event::WebhookEvent;

pub async fn start_github_webhook() -> anyhow::Result<Server> {
    //let secret = env::var("GITHUB_SECRET")?;
    //let port = env::var("GITHUB_WEBHOOK_PORT")?;

    let mut app = HttpServer::new(move || {
        App::new()
            .wrap(VerifyGithubSignatureFactory)
            .service(web::scope("/v1/github")
                .app_data(JsonConfig::default())
                .service(github_webhook_handler)
                .service(github_raw_webhook_handler)
            )
    });

    app = app.bind(("127.0.0.1",8080))?;

    Ok(app.run())
}

// Leaving this in for testing ect. TODO: Remove
const MAX_SIZE: usize = 262_144;
#[post("/webhook/raw")]
async fn github_raw_webhook_handler(mut payload: web::Payload) -> impl Responder {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    let body = decode(&body_str.as_str());

    println!("Received webhook: {:?}", body);
    println!("======================================");
    println!("======================================");
    Ok(HttpResponse::Ok().json("object"))
}
#[post("/webhook")]
async fn github_webhook_handler(event: web::Json<WebhookEvent>) -> Result<String> {
    
    println!("Received webhook: {:?}", event.into_inner());
    println!("======================================");
    println!("======================================");

    Ok("Ok".into())
}