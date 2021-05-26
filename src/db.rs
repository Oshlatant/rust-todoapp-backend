use std::{sync::Mutex};
use mongodb::{Client, bson::{Document}, Cursor};
use futures::StreamExt;


pub struct Database {
    pub content: Mutex<Client>,
}

pub async fn to_vector(cursor: &mut Cursor<Document>) -> Vec<Document> {

	let mut document_list = Vec::new();

	while let Some(document) = cursor.next().await {
		document_list.push(document.unwrap());
	}

	document_list
}
