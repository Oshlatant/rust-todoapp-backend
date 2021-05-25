use super::db::{self, random_id, Database, format_cursor};
use super::schemas::{ApiResponse, Todo};
use super::utils;
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use futures::StreamExt;
use mongodb::{ bson::{self, doc, Document}};

pub fn todos_config(cfg: &mut web::ServiceConfig) {
	cfg.service(web::resource("")
		.route(web::get().to(get_todos))
		.route(web::post().to(post_todo))
	);

    cfg.service(
        web::resource("/{id}/")
            .route(web::get().to(get_todo))
            .route(web::patch().to(patch_todo))
            .route(web::delete().to(delete_todo))
    );
}

async fn get_todos(db: web::Data<Database>) -> impl Responder {
    let todo_list = db.content.lock().unwrap();

	let mut todo_list = todo_list.find(None, None).await.unwrap();
	
	let todo_list = format_cursor(&mut todo_list).await;

    let response = ApiResponse {
        status: "success".to_string(),
        data: todo_list,
    };

    HttpResponse::Ok().json(response)
}

async fn get_todo(web::Path(id): web::Path<i32>, db: web::Data<Database>) -> impl Responder {
    //init
    let todo_list = db.content.lock().unwrap();

    //remove the todo from db state if found
    let todo = todo_list.find_one(doc! {"id": id}, None).await.unwrap();

    match todo {
        Some(todo) => {
            let response = ApiResponse {
                status: "success".to_string(),
                data: todo,
            };

            HttpResponse::Found().json(response)
        }
        None => utils::todo_not_found(),
    }
}


async fn post_todo(db: web::Data<Database>, todo: web::Json<Todo>) -> impl Responder {
    let todo_list = db.content.lock().unwrap();

	let todo = bson::to_document(&todo.0).expect("failed to convert");
	
    let result = todo_list.insert_one(todo.clone(), None).await;
	
	match result {
		Ok(result) => {

			println!(" ???? {:?}", result);

			let response = ApiResponse {
				status: "success".to_string(),
				data: todo,
			};
	
			HttpResponse::Created().json(response)
		},
		Err(e) => {

			eprintln!("error when post: {}", e);

			let response = ApiResponse {
				status: "failure".to_string(),
				data: serde_json::Value::Null,
			};

			HttpResponse::InternalServerError().json(response)
		}
	}
}

async fn patch_todo(
    web::Path(id): web::Path<i32>,
    db: web::Data<Database>,
    patched_todo: web::Json<Todo>,
) -> impl Responder {
    //init
    let todo_list = db.content.lock().unwrap();

	let update = bson::to_document(&patched_todo.0).expect("failed to convert");


    let result = todo_list.find_one_and_update(doc! {"id": id}, update , None).await;

	match result {
		Ok(todo) => {
			match todo {
				Some(todo) => {
					let response = ApiResponse {
						status: "success".to_string(),
						data: todo,
					};

					HttpResponse::Created().json(response)
				}
				None => {
					let response = ApiResponse {
						status: "failure".to_string(),
						data: serde_json::Value::Null,
					};
		
					HttpResponse::NotFound().json(response)
				}
			}

		},
		Err(_) => {
			let response = ApiResponse {
				status: "failure".to_string(),
				data: serde_json::Value::Null,
			};

			HttpResponse::InternalServerError().json(response)
		}
	}
}

async fn delete_todo(web::Path(id): web::Path<i32>, db: web::Data<Database>) -> impl Responder {
    let todo_list = db.content.lock().unwrap();

    let result = todo_list.find_one_and_delete(doc! { "id": id}, None).await;

    match result {
        Ok(_) => {
            let response = ApiResponse {
                status: "success".to_string(),
                data: serde_json::Value::Null,
            };

            HttpResponse::Accepted().json(response)
        }
        Err(_) => {
			let response = ApiResponse {
				status: "fail".to_string(),
				data: serde_json::Value::Null,
			};

			HttpResponse::InternalServerError().json(response)
		}
    }
}
