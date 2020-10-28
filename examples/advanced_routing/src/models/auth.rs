use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Default)]
/// Base Credential used for user authentication
pub struct AuthData {
    email: String,
    username: String,
    password: String,
}
/// Setters and getters for password
impl AuthData {
    pub fn set_password(&mut self, pwd: String) {
        self.password = pwd
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct LoginCredentials {
    target: String,
    password: String,
}

impl LoginCredentials {
    pub fn target(&self) -> &str {
        &self.target
    }

    pub fn password(&self) -> &str {
        &self.password
    }
    /// Set email or username
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
}
