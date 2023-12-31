use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub password: String,
    pub mode_of_login: String,
    pub merchant_name: String,
    pub user_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserRequest {
    pub password: String,
    pub mode_of_login: String,
    pub merchant_name: String,
    pub user_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserUpdateRequest {
    pub password: String,
    pub mode_of_login: String,
    pub merchant_name: String,
}
