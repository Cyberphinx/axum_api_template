use serde::{Deserialize, Serialize};

pub mod create;
pub mod login;
pub mod logout;

pub struct UserQueries;

/// User HTTP JSON Response object
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataUser {
    pub data: ResponseUser,
}

/// User HTTP Response object
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseUser {
    pub id: i32,
    pub email: String,
    pub role: String,
    pub token: String,
}

/// User Signup HTTP Request object
#[derive(Serialize, Deserialize)]
pub struct RequestCreateUser {
    pub email: String,
    pub password: String,
}

/// User Login HTTP Request object
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestLoginUser {
    pub email: String,
    pub password: String,
}

/// User Logout HTTP Response object
#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseLogoutUser {
    pub email: String,
    pub message: String,
}

/// constructor function to create a new instance of RequestLoginUser
impl RequestLoginUser {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
