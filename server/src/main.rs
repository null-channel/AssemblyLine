mod config;
mod database;
mod github;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use database::connection_manager::ConnectionManager;
use github::webhook::start_github_webhook;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace");

    // Get the server config

    // Get the database connection manager
    //let con_man = ConnectionManager::new(env::var("DATABASE_URL")?).await?;

    let _gh = start_github_webhook().await?.await;

    let con_man = ConnectionManager::new(env::var("DATABASE_URL")?).await?;

    let mut con = con_man.pool.acquire().await?;

    let id = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS repositories (
            id INTEGER PRIMARY KEY,
            description TEXT NOT NULL
        )
        "#,
    )
    .execute(&mut con)
    .await?
    .last_insert_rowid();

    // start listening to github webhook

    // start listening for clients
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
        .map_err(|e| e.into())
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
