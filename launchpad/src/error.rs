use midir::{ConnectError, InitError, PortInfoError, SendError};

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Midi(Box<dyn std::error::Error>),
    InvalidLocation,
    NoDevicesFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Midi(err) => write!(f, "midi error: {}", err),
            Self::InvalidLocation => write!(f, "invalid location"),
            Error::NoDevicesFound => write!(f, "No midi devices found"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<InitError> for Error {
    fn from(err: InitError) -> Error {
        Error::Midi(Box::new(err))
    }
}

impl From<PortInfoError> for Error {
    fn from(err: PortInfoError) -> Error {
        Error::Midi(Box::new(err))
    }
}

impl<T: 'static> From<ConnectError<T>> for Error {
    fn from(err: ConnectError<T>) -> Error {
        Error::Midi(Box::new(err))
    }
}

impl From<SendError> for Error {
    fn from(err: SendError) -> Error {
        Error::Midi(Box::new(err))
    }
}
