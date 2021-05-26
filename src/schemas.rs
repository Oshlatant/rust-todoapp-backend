use serde::{Deserialize, Serialize};
use mongodb::bson::{self, Document, from_document};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
	pub error: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateChecked {
	pub checked: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct ClientTodo {
    pub content: Option<String>,
    pub checked: Option<bool>,
}
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub content: Option<String>,
    pub checked: Option<bool>,
	pub id: Option<String>,
}

impl ClientTodo {

	pub fn to_todo(self, id: String) -> Todo {
		Todo {
			content: self.content,
			checked: self.checked,
			id: Some(id),
		}
	}

}


impl Todo {
	pub fn from(document: Document, id: Option<&bson::oid::ObjectId>) -> Todo {
		let id = match id {
			Some(id) => id.to_string(),
			None => document.get_object_id("_id").expect("no id").to_string(),
		};

		let todo: ClientTodo = from_document(document).expect("failed to convert todo");

		todo.to_todo(id)
	}
}

impl Clone for ClientTodo {
    fn clone(&self) -> Self {
        ClientTodo {
            content: self.content.clone(),
            checked: self.checked,
        }
    }
}

