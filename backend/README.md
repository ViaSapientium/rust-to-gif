# Rust to GIF Backend

## Description

This project aims to build a backend API in Rust to handle video files, specifically extracting images from videos, and providing an authentication service with PostgreSQL. The backend uses several modern technologies such as **Actix-Web**, **Deadpool PostgreSQL**, and **Argon2** for password hashing.

### Project Structure

- **Actix-Web**: Web framework used to manage routes and HTTP requests.
- **Deadpool PostgreSQL**: Connection pool manager for PostgreSQL.
- **Argon2**: Algorithm used to securely hash passwords.
- **FFmpeg**: Used to extract images from video files.

## Features Implemented

### Authentication

We have implemented an authentication system that includes:

- **User registration**: When registering, the email, username, and password are stored. Passwords are securely hashed using **Argon2**.
- **User login**: The system checks passwords using **Argon2** and generates a **JWT** token for user authentication.
- **JWT token management**: We implemented JWT generation and validation to secure API endpoints.

### PostgreSQL Connection

- **Connection test**: We created a function to test the connection to a PostgreSQL database using **Deadpool PostgreSQL**.
- **CRUD operations for users**:
  - Adding a user during registration.
  - Finding a user by login or email during login.
  - Deleting and updating users (prepared but not used yet).

### Video Management

- **Image extraction**: From a video file, we use **FFmpeg** to extract key images and store them in a specific directory.

## Technologies Used

- **Rust**: Main language used for backend development.
- **Actix-Web**: Framework to build performant APIs.
- **Deadpool PostgreSQL**: PostgreSQL connection management.
- **Argon2**: Secure password hashing.
- **FFmpeg**: Used for video file manipulation.
- **Tokio**: Asynchronous runtime for handling IO operations.

## Current Modules

### Auth Module

The `auth` module handles everything related to authentication:

- **auth_controller.rs**: Defines the `/login` and `/register` endpoints.
- **auth_service.rs**: Contains the authentication logic, such as password verification and JWT generation.

### User Module

The `user` module handles user information and actions:

- **user.rs**: Defines the user structure and methods for manipulating data in PostgreSQL (find, add, delete, update).

### Common Module

The `common` module contains shared utilities, like standardized responses:

- **responses.rs**: Defines standard HTTP responses for handling errors and successes (e.g., `bad_request`, `forbidden`, `unprocessable_entity`).

### PostgreSQL Module

The `postgres` module handles database configuration and connection management:

- **config.rs**: Sets up the PostgreSQL connection pool using **Deadpool**.
- **lib.rs**: Contains utility functions such as testing the PostgreSQL connection.

## Next Steps

- Add password reset token management.
- Implement a notification service.
- Improve error handling and HTTP response messages.
- Develop a user session management system.
- Optimize video image extraction to support various formats and resolutions.

# Backend Progress Overview

### 1. Project Initialization

- **Framework**: We set up the backend using **Rust** and integrated it with **Actix-web** as our web framework.
- **Database**: The database choice was **PostgreSQL**, and we used **Deadpool** for connection pooling.
- **Key Libraries**:
  - `tokio_postgres`
  - `serde` for serialization
  - `argon2` for password hashing
  - `uuid` for generating unique IDs
  - `chrono` for handling dates

### 2. Authentication System

- We implemented a **JWT-based authentication** system.
- **Password hashing**: We used the **Argon2** algorithm to ensure secure password storage.
- **JWT Token Generation**: Tokens are generated for both login and registration using `jsonwebtoken`.
- Routes:
  - `/login`: Handles user login and returns a JWT token upon success.
  - `/register`: Allows new users to create accounts, with the password hashed using Argon2.

### 3. Database Integration

- **Database Connection**: We used `Deadpool` for pooling connections to the PostgreSQL database.
- **User Management**: Implemented user registration, login, and password management.
- Database operations such as inserting and retrieving user data were abstracted using **tokio_postgres** and custom traits for `UserMethods`.

### 4. Error Handling and API Responses

- Created a unified API response system using a custom `ApiResponse` struct.
- Implemented various error responses:
  - `200 OK`
  - `201 Created`
  - `204 No Content`
  - `400 Bad Request`
  - `401 Unauthorized`
  - `403 Forbidden`
  - `404 Not Found`
  - `409 Conflict`
  - `422 Unprocessable Entity`
  - `500 Internal Server Error`

### 5. Password Reset Feature

- Developed a **password reset** mechanism:
  - Tokens are generated and stored with an expiration time.
  - Users can request password resets, which will verify the token before allowing them to change their password.
