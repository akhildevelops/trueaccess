use crate::TAResult;

pub struct TAUser {
    username: String,
    password: String,
}

impl TAUser {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub trait User {
    fn create_user(&self, username: &str, password: &str) -> TAResult<TAUser>;
}
