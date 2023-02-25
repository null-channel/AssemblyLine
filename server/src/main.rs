mod config;
mod database;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use database::connection_manager::ConnectionManager;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get the server config

    // Get the database connection manager

    // start listening to github webhook

    // start listening for clients
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/agent/{id}")]
async fn hello() -> impl Responder {
    // if nothing to do
    HttpResponse::Ok().body("Hello world!")

    // if something to do
    // return the pipeline to run
}

#[post("/agent")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
