use super::schemas::{ApiResponse, ApiError};
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