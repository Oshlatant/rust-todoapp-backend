use serde::{Deserialize, Serialize};

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
pub struct Todo {
    pub content: Option<String>,
    pub checked: Option<bool>,
    pub _id: Option<i32>,
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            content: self.content.clone(),
            checked: self.checked,
            _id: self._id,
        }
    }
}

