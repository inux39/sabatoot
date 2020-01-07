use std::fmt;
use std::string::FromUtf8Error;
use std::io::Error as IoError;
use mammut::Error as MammutError;
use toml::de::Error as TomlDeError;
use toml::ser::Error as TomlSerError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Mammut(MammutError),
    TomlDe(TomlDeError),
    TomlSer(TomlSerError),
    Utf8(FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

impl From<MammutError> for Error {
    fn from(error: MammutError) -> Self {
        Error::Mammut(error)
    }
}

impl From<TomlDeError> for Error {
    fn from(error: TomlDeError) -> Self {
        Error::TomlDe(error)
    }
}

impl From<TomlSerError> for Error {
    fn from(error: TomlSerError) -> Self {
        Error::TomlSer(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Self {
        Error::Utf8(error)
    }
}

