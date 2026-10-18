
pub enum Error {
    Again,
}

pub type Result<T> = std::result::Result<T, Error>;

