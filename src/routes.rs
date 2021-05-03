use super::db::{random_id, JsonDb};
use super::schemas::{JsonError, Todo};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use std::fs;

#[get("/todos")]
async fn get_todos(db: web::Data<JsonDb>) -> impl Responder {
    let db_content = db.content.lock().unwrap();

    HttpResponse::Ok().body(&*db_content)
}

#[get("/todos/{id}")]
async fn get_todo(web::Path(id): web::Path<i32>, db: web::Data<JsonDb>) -> impl Responder {
    let db_content = db.content.lock().unwrap();
    let todos_list: Vec<Todo> = serde_json::from_str(&*db_content).unwrap();

    //find the todo with the good id and make it as string
    let wanted_todo = todos_list.iter().find(|todo| todo._id == Some(id));
    let wanted_todo = wanted_todo.map(|t| serde_json::to_string(t).unwrap());

    //respond the todo if found
    match wanted_todo {
        Some(todo) => HttpResponse::Ok().body(todo),
        None => {
            let json_error = serde_json::to_string(&JsonError {
                error: "Todo not found".to_string(),
            })
            .unwrap();
            HttpResponse::NotFound().body(json_error)
        }
    }
}

#[post("/todos")]
async fn post_todo(db: web::Data<JsonDb>, mut todo: web::Json<Todo>) -> impl Responder {
    //init
    todo._id = Some(random_id());
    let mut db_string = db.content.lock().unwrap();
    let mut db: Vec<Todo> = serde_json::from_str(&*db_string).unwrap();


	println!("{}", todo.content);

    //update db string
    db.push(todo.0);
    let updated_db_string = serde_json::to_string(&db).unwrap();
    *db_string = updated_db_string.clone();

    //write in db.json
    web::block(|| fs::write("./db.json", updated_db_string))
        .await
        .unwrap();

    //give the last todo ( new one ) as response
    let response = serde_json::to_string(&db.last()).unwrap();
    HttpResponse::Created().body(response)
}

// #[patch("/todos/{id}")]
// async fn patch_todo() -> impl HttpResponse {

// }

// #[delete("/todos/{id}")]
// async fn delete_todo() -> impl HttpResponse {

// }
