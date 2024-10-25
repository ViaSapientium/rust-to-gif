use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
  message: String,
  data: Option<serde_json::Value>,
}

impl ApiResponse {
  // 200 OK
  pub fn success(message: &str, data: Option<serde_json::Value>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
      message: message.to_string(),
      data,
    })
  }

  // 201 Created
  pub fn created(message: &str, data: Option<serde_json::Value>) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse {
      message: message.to_string(),
      data,
    })
  }

  // 204 No Content
  pub fn no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
  }

  // 209 Custom Content
  pub fn content(message: &str, data: Option<serde_json::Value>) -> HttpResponse {
    HttpResponse::build(StatusCode::from_u16(209).unwrap()).json(ApiResponse {
      message: message.to_string(),
      data,
    })
  }

  // 400 Bad Request
  pub fn bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 401 Unauthorized
  pub fn unauthorized(message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 403 Forbidden
  pub fn forbidden(message: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 404 Not Found
  pub fn not_found(message: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 409 Conflict
  pub fn conflict(message: &str) -> HttpResponse {
    HttpResponse::Conflict().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 422 Unprocessable Entity
  pub fn unprocessable_entity(message: &str) -> HttpResponse {
    HttpResponse::UnprocessableEntity().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // 500 Internal Server Error
  pub fn internal_server_error(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse {
      message: message.to_string(),
      data: None,
    })
  }

  // from_error to transform an error into a response
  pub fn from_error<E: std::fmt::Display>(error: E) -> HttpResponse {
    HttpResponse::InternalServerError().json(ApiResponse {
      message: error.to_string(),
      data: None,
    })
  }
}
