mod routes;
mod db;
mod schemas;

use std::sync::Mutex;
use std::fs;
use actix_web::{App, HttpServer, web};
use actix_cors::Cors;

use routes::{get_todos, get_todo, post_todo};
use db::{JsonDb};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let json_db = {
		// get json file as string ( acting like a db )
		let db = web::block(|| {
			fs::read_to_string("./db.json")
		}).await.unwrap();

		web::Data::new( JsonDb {
			content: Mutex::new(db),
		})
	};

    
	
    HttpServer::new(move || {
		
		
		let cors = Cors::default()
		.send_wildcard();



        App::new()
            .wrap(cors)
            .app_data(json_db.clone())
            .service(get_todos)
            .service(get_todo)
            .service(post_todo)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}