use ssh2::Error as sshError;
use std::error::Error;
use std::fmt::Display;
use std::io::Error as ioError;
#[derive(Debug)]
pub enum TAError {
    UserCreationError,
    TCPConnectionError,
    SSHModuleError(String),
}

impl Display for TAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserCreationError => write!(f, "Error in Creation of User"),
            Self::TCPConnectionError => write!(f, "Error in establising a tcp connection"),
            Self::SSHModuleError(x) => write!(f, "Error in ssh module: {}", x),
        }
    }
}

impl Error for TAError {}

impl From<ioError> for TAError {
    fn from(x: ioError) -> Self {
        Self::TCPConnectionError
    }
}

impl From<sshError> for TAError {
    fn from(x: sshError) -> Self {
        Self::SSHModuleError(x.message().to_string())
    }
}
