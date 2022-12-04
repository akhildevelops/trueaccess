use crate::user::{TAUser, User};
use crate::TAResult;
use ssh2::Session;
use std::net::TcpStream;
struct Linux {
    username: String,
    password: String,
    hostname: String,
    port: u32,
}

impl Linux {
    fn new(username: &str, password: &str, hostname: &str, port: u32) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            hostname: hostname.to_string(),
            port: port,
        }
    }

    fn create_session(&self) -> TAResult<LinuxSession> {
        let mut session = ssh2::Session::new()?;
        let tcp = TcpStream::connect(format!("{}:{}", self.hostname, self.port))?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(&self.username, &self.password)?;
        Ok(LinuxSession(session))
    }
}

struct LinuxSession(Session);

impl LinuxSession {
    fn check_connection(&mut self) -> TAResult<bool> {
        Ok(self.0.authenticated())
    }
}

impl User for LinuxSession {
    fn create_user(&self, username: &str, password: &str) -> TAResult<TAUser> {
        let mut channel = self.0.channel_session()?;
        let user_add = format!("useradd {}", username);
        let change_passwd = format!("passwd {}", username);
        channel.exec(&user_add)?;

        // channel.exec(&change_passwd)?;

        // channel.exec(&password)?;
        // channel.exec(&password)?;

        Ok(TAUser::new(username, password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const hostname: &str = "172.17.0.2";
    const username: &str = "root";
    const password: &str = "mypassword";
    const port: u32 = 22;
    #[test]
    fn check_connection() {
        let linux = Linux::new(username, password, hostname, port);
        assert_eq!(
            linux.create_session().unwrap().check_connection().unwrap(),
            true
        )
    }

    #[test]
    fn create_user() {
        let linux = Linux::new(username, password, hostname, port);
        linux
            .create_session()
            .unwrap()
            .create_user("dumm2", "asdf")
            .unwrap();
    }
}
