use actix_web::{App, HttpServer};
mod base;
mod config;
mod handler;
mod handlers;
mod infrastructure;
mod model;
mod routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init config
    config::init().await;

    // init minio
    infrastructure::minio::init().await;

    HttpServer::new(|| App::new().service(routers::init()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
