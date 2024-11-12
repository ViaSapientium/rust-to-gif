use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub status: String,
  pub message: String,
  pub data: Option<T>,
}

impl<T> ApiResponse<T> {
  pub fn success(message: &str, data: Option<T>) -> ApiResponse<T> {
    ApiResponse {
      status: "success".to_string(),
      message: message.to_string(),
      data,
    }
  }

  pub fn error(message: &str, data: Option<T>) -> ApiResponse<T> {
    ApiResponse {
      status: "error".to_string(),
      message: message.to_string(),
      data,
    }
  }
}

#[derive(Serialize)]
pub struct DetailedErrorResponse {
  pub code: u16,
  pub message: String,
  pub details: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
  pub items: Vec<T>,
  pub total: u64,
  pub page: u32,
  pub page_size: u32,
}
