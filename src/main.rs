mod db;
mod routes;
mod schemas;
mod utils;

use std::env;

use actix_cors::Cors;
use actix_web::middleware::{normalize::NormalizePath, Logger};
use actix_web::{web, App, HttpServer};
use mongodb::Client;

use routes::todos_config;

const DB_IP: &str = "mongodb://localhost:27017/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	
	let client = Client::with_uri_str(DB_IP).await.expect("failed to connect");
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let ip = "0.0.0.0";
    // let ip = "localhost";

    println!("Server running on port: {}", port);

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::permissive()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(NormalizePath::default())
            .data(client.clone())
            .service(web::scope("/todos/").configure(todos_config))
            .service(
                web::resource("/test/")
                    .route(web::get().to(|| actix_web::HttpResponse::Ok().body("test"))),
            )
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
