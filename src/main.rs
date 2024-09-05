use actix_web::{App, HttpServer};
mod base;
mod handler;
mod handlers;
mod model;
mod routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(routers::init()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
