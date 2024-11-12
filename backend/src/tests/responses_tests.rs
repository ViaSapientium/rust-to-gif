#[cfg(test)]
mod tests {
  use super::ResponsesErrorCodes;
  use super::*;
  use actix_web::http::StatusCode;
  use serde_json::to_string;

  #[test]
  fn test_responses_error_codes() {
    assert_eq!(ResponsesErrorCodes::BadRequest.to_u16(), 400);
    assert_eq!(ResponsesErrorCodes::BadRequest.description(), "The query syntax is incorrect");
    let status: StatusCode = ResponsesErrorCodes::InternalServerError.into();
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
  }

  #[test]
  fn test_my_response_serialization() {
    let response = MyResponse {
      data: "Hello, World!".to_string(),
    };
    let serialized = to_string(&response).expect("Serialization failed");
    assert_eq!(serialized, r#"{"data":"Hello, World!"}"#);
  }
}
