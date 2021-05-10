use super::db::{self, random_id, JsonDb};
use super::schemas::{ApiResponse, Todo};
use super::utils;
use actix_web::web;
use actix_web::{HttpResponse, Responder};

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

async fn get_todos(db: web::Data<JsonDb>) -> impl Responder {
    let todo_list = db.content.lock().unwrap();

    let response = ApiResponse {
        status: "success".to_string(),
        data: todo_list.clone(),
    };

    HttpResponse::Ok().json(response)
}

async fn get_todo(web::Path(id): web::Path<i32>, db: web::Data<JsonDb>) -> impl Responder {
    //init
    let mut todo_list = db.content.lock().unwrap();

    //remove the todo from db state if found
    let todo = todo_list.remove_entry(&id.to_string());

    match todo {
        Some((id, todo)) => {
            //reinsert the todo previously removed
            todo_list.insert(id, todo.clone());

            let response = ApiResponse {
                status: "success".to_string(),
                data: todo,
            };

            HttpResponse::Found().json(response)
        }
        None => utils::todo_not_found(),
    }
}


async fn post_todo(db: web::Data<JsonDb>, mut todo: web::Json<Todo>) -> impl Responder {
    //init
    todo._id = Some(random_id());
    let mut todo_list = db.content.lock().unwrap();
    let response = ApiResponse {
        status: "success".to_string(),
        data: todo.0.clone(),
    };

    //update db hashmap
    todo_list.insert(todo._id.unwrap().to_string(), todo.0);

    //write in db.json
    let db_string = serde_json::to_string(&*todo_list).unwrap();
    db::update_db(db_string).await;

    HttpResponse::Created().json(response)
}

async fn patch_todo(
    web::Path(id): web::Path<i32>,
    db: web::Data<JsonDb>,
    patched_todo: web::Json<Todo>,
) -> impl Responder {
    //init
    let mut todo_list = db.content.lock().unwrap();

    let todo = todo_list.remove(&id.to_string());

    match todo {
        Some(mut todo) => {
            //patch the todo
            todo.checked = patched_todo.checked;

            //reinsert the todo patched
            todo_list.insert(id.to_string(), todo.clone());

            //update the db.json
            db::update_db(serde_json::to_string(&*todo_list).unwrap()).await;

            //response the todo patched
            let response = ApiResponse {
                status: "success".to_string(),
                data: todo,
            };

            HttpResponse::Ok().json(response)
        }
        None => utils::todo_not_found(),
    }
}

async fn delete_todo(web::Path(id): web::Path<i32>, db: web::Data<JsonDb>) -> impl Responder {
    let mut todo_list = db.content.lock().unwrap();

    let todo = todo_list.remove(&id.to_string());

    match todo {
        Some(_todo) => {
            //update the db.json
            db::update_db(serde_json::to_string(&*todo_list).unwrap()).await;

            let response = ApiResponse {
                status: "success".to_string(),
                data: serde_json::Value::Null,
            };
            //response the todo patched
            HttpResponse::Accepted().json(response)
        }
        None => utils::todo_not_found(),
    }
}
