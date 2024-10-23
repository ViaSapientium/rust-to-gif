use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;

pub struct ApiResponse;

impl ApiResponse {
  // 200 OK
  pub fn success(message: &str, data: Option<serde_json::Value>) -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": message,
        "data": data.unwrap_or_else(|| json!(null))
    }))
  }

  // 400 Bad Request
  pub fn bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(json!({
        "status": "error",
        "code": StatusCode::BAD_REQUEST.as_u16(),
        "message": message,
    }))
  }

  // 401 Unauthorized
  pub fn unauthorized(message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(json!({
        "status": "error",
        "code": StatusCode::UNAUTHORIZED.as_u16(),
        "message": message,
    }))
  }

  // 403 Forbidden
  pub fn forbidden(message: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(json!({
        "status": "error",
        "code": StatusCode::FORBIDDEN.as_u16(),
        "message": message,
    }))
  }

  // 404 Not Found
  pub fn not_found(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(json!({
        "status": "error",
        "code": StatusCode::NOT_FOUND.as_u16(),
        "message": message,
    }))
  }

  // 500 Internal Server Error
  pub fn internal_server_error(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({
        "status": "error",
        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        "message": message,
    }))
  }

  // 422 Unprocessable Entity
  pub fn unprocessable_entity(message: &str) -> HttpResponse {
    HttpResponse::UnprocessableEntity().json(json!({
        "status": "error",
        "code": StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
        "message": message,
    }))
  }

  // 409 Conflict
  pub fn conflict(message: &str) -> HttpResponse {
    HttpResponse::Conflict().json(json!({
        "status": "error",
        "code": StatusCode::CONFLICT.as_u16(),
        "message": message,
    }))
  }
}
