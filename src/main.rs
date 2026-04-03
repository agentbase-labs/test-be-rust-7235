use actix_web::{web, App, HttpServer, HttpResponse, get};
use serde_json::json;
use std::env;

#[get("/")]
async fn home() -> HttpResponse {
    HttpResponse::Ok().json(json!({"message": "Rust Works!"}))
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({"status": "ok", "framework": "rust"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let port: u16 = port.parse().unwrap();
    println!("Server on port {}", port);
    HttpServer::new(|| App::new().service(home).service(health))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}