mod db;
mod routes;
mod schemas;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::fs;
use std::sync::Mutex;

use db::JsonDb;
use routes::{get_todo, get_todos, post_todo};
use std::env;

use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_db = {
        // get json file as string ( acting like a db )
        let db = web::block(|| fs::read_to_string("./db.json"))
            .await
            .unwrap();

        web::Data::new(JsonDb {
            content: Mutex::new(db),
        })
    };

    let port = env::var("PORT").unwrap_or("3000".to_string());
	let ip = "0.0.0.0";
	
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
            .app_data(json_db.clone())
            .service(get_todos)
            .service(get_todo)
            .service(post_todo)
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
