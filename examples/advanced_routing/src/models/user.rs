use crate::models::auth::AuthData;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    #[serde(flatten)]
    pub credentials: AuthData,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoggedUser {
    pub first_name: String,
    pub last_name: String,
    username: String,
    email: String,
    pub role: Role,
}

impl LoggedUser {
    pub fn new(first_name: &str, last_name: &str, username: &str, email: &str, role: Role) -> Self {
        LoggedUser {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            role,
        }
    }
}

impl LoggedUser {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Role {
    StandardUser,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Role::StandardUser
    }
}
