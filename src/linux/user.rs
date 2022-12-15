use crate::error::TAError;
use crate::linux::utils;
use crate::user::{TAUser, User, UserError};
use crate::TAResult;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
pub struct Linux {
    username: String,
    password: String,
    hostname: String,
    port: u32,
}

impl Linux {
    pub fn new(username: &str, password: &str, hostname: &str, port: u32) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            hostname: hostname.to_string(),
            port: port,
        }
    }

    pub fn create_session(&self) -> TAResult<LinuxSession> {
        let mut session = ssh2::Session::new()?;
        let tcp = TcpStream::connect(format!("{}:{}", self.hostname, self.port))?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_password(&self.username, &self.password)?;
        // session.set_blocking(false);
        Ok(LinuxSession(session))
    }
}

pub struct LinuxSession(Session);

impl LinuxSession {
    fn check_connection(&mut self) -> TAResult<bool> {
        Ok(self.0.authenticated())
    }
    fn check_user(&self, user: &TAUser) -> TAResult<bool> {
        let mut channel = self.0.channel_session()?;
        let contents_cmd = format!("cat {}", utils::ETC_PASSWD);
        channel.exec(&contents_cmd)?;
        let mut contents = String::new();
        channel.read_to_string(&mut contents)?;
        channel.wait_close()?;
        channel.exit_status()?;

        Ok(contents.contains(&user.username))
    }
}

impl User for LinuxSession {
    fn create_user(&self, username: &str, password: &str) -> TAResult<TAUser> {
        let user = TAUser::new(username, password);
        let mut channel = self.0.channel_session()?;
        let user_add = format!("{} {}", utils::USER_ADD, username);
        // let change_passwd = format!("passwd {}", username);
        // channel.shell();
        channel.exec(&user_add)?;
        // thread::sleep(time::Duration::from_millis(1000));
        // dbg!(&channel.read_window().available);
        let mut stderr = channel.stderr();
        let mut response = String::new();
        // channel.wait_eof().unwrap();
        dbg!(stderr.read_to_string(&mut response).unwrap());

        dbg!(&response);

        // channel.send_eof().unwrap();
        // channel.close().unwrap();
        channel.wait_close()?;
        channel.exit_status()?;

        if utils::user_exist_from_response(&response, Some(&user)) {
            Err(TAError::UserError(UserError::AlreadyExists(user)))
        } else if self.check_user(&user)? {
            Ok(user)
        } else {
            Err(TAError::UserError(UserError::Other(user)))
        }

        // channel.exec(&change_passwd)?;

        // channel.exec(&password)?;
        // channel.exec(&password)?;
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

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
            .create_user("dumm23", "asdf")
            .unwrap();
        // panic!();
    }
}
