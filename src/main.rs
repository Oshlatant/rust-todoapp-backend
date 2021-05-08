mod db;
mod routes;
mod schemas;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use std::sync::Mutex;
use std::{fs};
use db::JsonDb;

use routes::{get_todo, get_todos, patch_todo, post_todo, delete_todo};
use std::env;

use actix_web::middleware::Logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_db = {
        // get json file as string ( acting like a db )
        let db = web::block(|| fs::read_to_string("./db.json"))
            .await
            .unwrap();

        // let json: Vec<Todo> = serde_json::from_str(&db).unwrap_or_else(|e| panic!("error: {}", e));
        let db = serde_json::from_str(&db).unwrap();

        web::Data::new(JsonDb {
            content: Mutex::new(db),
        })
    };

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let ip = "0.0.0.0";
    // let ip = "localhost";

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
            .service(patch_todo)
            .service(delete_todo)
    })
    .bind(format!("{}:{}", ip, port))?
    .run()
    .await
}
