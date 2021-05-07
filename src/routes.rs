use super::db::{self, random_id, JsonDb};
use super::schemas::{ Todo};
use actix_web::web::{self, Json};
use actix_web::{delete, get, patch, post, HttpResponse, Responder};

#[get("/todos")]
async fn get_todos(db: web::Data<JsonDb>) -> impl Responder {
    let todo_list = db.content.lock().unwrap();

    let todo_list = db::todo_hashmap_vec(&todo_list);
    let todo_list = serde_json::to_string(&todo_list).unwrap();

    HttpResponse::Ok().body(todo_list)
}

#[get("/todos/{id}")]
async fn get_todo(
    web::Path(id): web::Path<i32>,
    db: web::Data<JsonDb>,
) -> actix_web::Result<Json<Todo>> {
    //init
    let mut todo_list = db.content.lock().unwrap();

    //find the todo with the good id and make it as string
    // let wanted_todo = todo_list.iter().find(|todo| todo._id == Some(id));
    // let wanted_todo = wanted_todo.map(|t| serde_json::to_string(t).unwrap());

    let todo = todo_list.remove_entry(&id.to_string());

    match todo {
        Some((id, todo)) => {
            todo_list.insert(id, todo.clone());
            Ok(Json(todo))
        }
        None => Ok(Json(Todo {
            content: Some("Error".to_string()),
            checked: Some(false),
            _id: Some(id),
        })),
    }
}

#[post("/todos")]
async fn post_todo(db: web::Data<JsonDb>, mut todo: web::Json<Todo>) -> impl Responder {
    //init
    todo._id = Some(random_id());
    let mut todo_list = db.content.lock().unwrap();
    let response = serde_json::to_string(&todo.0).unwrap();

    //update db hashmap
    todo_list.insert(todo._id.unwrap().to_string(), todo.0);

    //write in db.json
    let db_string = serde_json::to_string(&*todo_list).unwrap();
    db::update_db(db_string).await;

    HttpResponse::Created().body(response)
}

#[patch("/todos/{id}")]
async fn patch_todo(
    web::Path(id): web::Path<i32>,
    db: web::Data<JsonDb>,
    patched_todo: web::Json<Todo>,
) -> actix_web::Result<Json<Todo>> {
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
            Ok(Json(todo))
        }
        None => Ok(Json(Todo {
            content: Some("Error".to_string()),
            checked: Some(false),
            _id: Some(id),
        })),
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(
	web::Path(id): web::Path<i32>,
	db: web::Data<JsonDb>,
) -> impl Responder {
	let mut todo_list = db.content.lock().unwrap();

    let todo = todo_list.remove(&id.to_string());

    match todo {
        Some( _todo) => {
			//update the db.json 
			db::update_db(serde_json::to_string(&*todo_list).unwrap()).await;

			//response the todo patched
            HttpResponse::Accepted()
        }
        None => HttpResponse::NotFound(),
    }
}
