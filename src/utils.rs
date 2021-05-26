use super::schemas::{ApiError, ApiResponse};
use actix_web::HttpResponse;

pub fn todo_not_found() -> HttpResponse {
    let error = ApiError {
        error: "Todo not found".to_string(),
    };

    let response = ApiResponse {
        status: "fail".to_string(),
        data: error,
    };

    HttpResponse::NotFound().json(response)
}

// pub fn json_to_doc<T: Serialize>(json: T) -> Document {
// 	let document = bson::to_document(&json).expect("failed to convert json to doc");

// 	document
// }
