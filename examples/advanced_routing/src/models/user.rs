use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct LoggedData {
    pub first_name: String,
    pub last_name: String,
    username: String,
    email: String,
    pub role: Role,
}

impl LoggedData {
    pub fn new(first_name: &str, last_name: &str, username: &str, email: &str, role: Role) -> Self {
        LoggedData {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            role,
        }
    }
}

impl LoggedData {
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
