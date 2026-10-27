use std::fmt;

#[derive(Debug)]
pub enum Error {
    Again,
    Msg(String),
    ServiceShutdown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Again => write!(f, "try again"),
            Self::Msg(s) => write!(f, "{s}"),
            Self::ServiceShutdown => write!(f, "service has shutdown"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! e_fmt {
    ($($arg:tt)+) => {
        Error::Msg(format!($($arg)+))
    };
}

pub(crate) use e_fmt;

