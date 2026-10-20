
#[derive(Debug)]
pub enum Error {
    Again,
    Msg(String),
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! e_fmt {
    ($($arg:tt)+) => {
        Error::Msg(format!($($arg)+))
    };
}

pub(crate) use e_fmt;

