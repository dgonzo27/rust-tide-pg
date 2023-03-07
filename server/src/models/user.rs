use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserPartial {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserLoginResponse {
    pub access_token: String,
}
