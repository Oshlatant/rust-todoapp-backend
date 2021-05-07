use random_number::random_ranged;
use std::{collections::HashMap, sync::Mutex, fs};
use actix_web::web;

use super::schemas::Todo;

pub struct JsonDb {
    pub content: Mutex<HashMap<String, Todo>>,
}

pub fn random_id() -> i32 {
    random_ranged(1..=10000)
}

pub fn todo_vec_hashmap(vec: &Vec<Todo>) -> HashMap<String, Todo> {

	let mut hashmap: HashMap<String, Todo> = HashMap::new();

	for todo in vec.iter() {
		hashmap.insert(format!("{}", todo._id.unwrap()), todo.clone());
	}

	hashmap
}
pub fn todo_hashmap_vec(hashmap: &HashMap<String, Todo>) -> Vec<Todo> {

	let mut vec = Vec::new();

	for (_id, todo) in hashmap.iter() {
		vec.push(todo.clone());
	}

	vec
}

pub async fn update_db(db: String) {
	web::block(|| fs::write("./db.json", db)).await.unwrap();
}