use crate::schemas::UpdateChecked;

use super::db;
use super::schemas::{ApiResponse, ClientTodo, Todo};
use super::utils;
use actix_web::web;
use actix_web::{HttpResponse, Responder};

use mongodb::bson::oid;
use mongodb::bson::{self, doc};
use mongodb::Client;

const DATABASE: &str = "todoapp";
const COLLECTION: &str = "todos";

pub fn todos_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(get_todos))
            .route(web::post().to(post_todo)),
    );

    cfg.service(
        web::resource("/{id}/")
            .route(web::get().to(get_todo))
            .route(web::patch().to(patch_todo))
            .route(web::delete().to(delete_todo)),
    );
}

async fn get_todos(client: web::Data<Client>) -> impl Responder {
    let todo_list = client
        .database(DATABASE)
        .collection(COLLECTION);

    let mut todo_list = todo_list.find(None, None).await.unwrap();

    let todo_list = db::to_vector(&mut todo_list).await;

    let todo_list: Vec<Todo> = todo_list
        .iter()
        .map(|document| Todo::from(document.clone(), None))
        .collect();

    let response = ApiResponse {
        status: "success".to_string(),
        data: todo_list,
    };

    HttpResponse::Ok().json(response)
}

async fn get_todo(web::Path(id): web::Path<String>, client: web::Data<Client>) -> impl Responder {
    //init
    let todo_list = client
        .database(DATABASE)
        .collection(COLLECTION);

    let id = oid::ObjectId::with_string(&id).expect("failed to id");

    let todo = todo_list
        .find_one(Some(doc! {"_id": &id}), None)
        .await
        .unwrap();

    match todo {
        Some(todo) => {
            let response = ApiResponse {
                status: "success".to_string(),
                data: Todo::from(todo, Some(&id)),
            };

            println!("found");

            HttpResponse::Found().json(response)
        }
        None => utils::todo_not_found(),
    }
}

async fn post_todo(client: web::Data<Client>, todo: web::Json<ClientTodo>) -> impl Responder {
    let todo_list = client
        .database(DATABASE)
        .collection(COLLECTION);
    let todo = bson::to_document(&todo.0).expect("failed to convert");

    let result = todo_list.insert_one(todo.clone(), None).await;

    match result {
        Ok(result) => {
            let id = result.inserted_id.as_object_id();
            let todo = Todo::from(todo, id);

            let response = ApiResponse {
                status: "success".to_string(),
                data: todo,
            };

            HttpResponse::Created().json(response)
        }
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
    web::Path(id): web::Path<String>,
    client: web::Data<Client>,
    patched_todo: web::Json<UpdateChecked>,
) -> impl Responder {
    //init
    let todo_list = client
        .database(DATABASE)
        .collection(COLLECTION);

    let update = doc! { "$set": {"checked": patched_todo.checked.unwrap()} };

    let id = oid::ObjectId::with_string(&id).expect("failed to id");

    println!("{:?}", update);

    let result = todo_list
        .find_one_and_update(doc! {"_id": &id}, update, None)
        .await;

    match result {
        Ok(todo) => match todo {
            Some(todo) => {
                let response = ApiResponse {
                    status: "success".to_string(),
                    data: Todo::from(todo, Some(&id)),
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
        },
        Err(e) => {
            println!("fail fail fail {}", e);

            let response = ApiResponse {
                status: "failure".to_string(),
                data: serde_json::Value::Null,
            };

            HttpResponse::InternalServerError().json(response)
        }
    }
}

async fn delete_todo(web::Path(id): web::Path<String>, client: web::Data<Client>) -> impl Responder {
    let todo_list = client
        .database(DATABASE)
        .collection(COLLECTION);
    let id = oid::ObjectId::with_string(&id).expect("failed to id");
    let result = todo_list.find_one_and_delete(doc! { "_id": id}, None).await;

    match result {
        Ok(_) => {
            let response = ApiResponse {
                status: "success".to_string(),
                data: serde_json::Value::Null,
            };

            HttpResponse::Accepted().json(response)
        }
        Err(e) => {
            eprintln!("{}", e);

            let response = ApiResponse {
                status: "fail".to_string(),
                data: serde_json::Value::Null,
            };

            HttpResponse::InternalServerError().json(response)
        }
    }
}
