use actix_web::web;
use random_number::random_ranged;
use std::{collections::HashMap, fs, sync::Mutex};
use mongodb::{Client, Collection, bson::{Document, doc}, Cursor};
use futures::StreamExt;
use super::schemas::Todo;

pub struct Database {
    pub content: Mutex<Collection<Document>>,
}

pub fn random_id() -> i32 {
    random_ranged(1..=10000)
}

// pub fn todo_hashmap_vec(hashmap: &HashMap<String, Todo>) -> Vec<Todo> {

// 	let mut vec = Vec::new();

// 	for (_id, todo) in hashmap.iter() {
// 		vec.push(todo.clone());
// 	}

// 	vec
// }


pub async fn format_cursor(cursor: &mut Cursor<Document>) -> Vec<Document> {

	let mut document_list = Vec::new();

	while let Some(document) = cursor.next().await {
		document_list.push(document.unwrap());
	}

	document_list
}


pub async fn update_db(db: String) {
    web::block(|| fs::write("./db.json", db)).await.unwrap();
}
