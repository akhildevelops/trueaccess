use crate::error::TAError;
use crate::TAResult;
use std::fmt::Debug;
pub struct TAUser {
    pub username: String,
    password: Option<String>,
}

impl Debug for TAUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TAUser({})", self.username)
    }
}

impl TAUser {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: Some(password.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum UserError {
    AlreadyExists(TAUser),
    Other(TAUser),
}

pub trait User {
    fn create_user(&self, username: &str, password: &str) -> TAResult<TAUser>;
}

impl From<UserError> for TAError {
    fn from(x: UserError) -> Self {
        Self::UserError(x)
    }
}
