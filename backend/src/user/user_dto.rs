use serde::{Deserialize, Serialize};

// DTO for creating a new user
#[derive(Deserialize)]
pub struct CreateUserRequest {
  pub username: String,
  pub login: String,
  pub email: String,
  pub password: String,
}

// DTO for updating user details
#[derive(Deserialize)]
pub struct UpdateUserRequest {
  pub username: Option<String>,
  pub email: Option<String>,
  pub password: Option<String>,
}

// DTO for updating user password
#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
  pub email: String,
  pub new_password: String,
}

// DTO for user response with user details
#[derive(Serialize)]
pub struct UserResponse {
  pub id: i32,
  pub username: String,
  pub login: String,
  pub email: String,
}

// DTO for responses with a list of users
#[derive(Serialize)]
pub struct UsersResponse {
  pub users: Vec<UserResponse>,
}

// DTO for responses with a single user
#[derive(Serialize)]
pub struct SingleUserResponse {
  pub user: UserResponse,
}

// DTO for responses with a message specific to user actions
#[derive(Serialize)]
pub struct UserMessageResponse {
  pub message: String,
}

// DTO for general error responses in user context
#[derive(Serialize)]
pub struct UserErrorResponse {
  pub error: String,
}
