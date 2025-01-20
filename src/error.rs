pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRegion(String),
    InvalidPath(std::path::PathBuf),
    ListResponseError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidRegion(region) => write!(f, "Invalid region: {}", region),
            Error::InvalidPath(path) => write!(f, "Invalid path: {:?}", path),
            Error::ListResponseError(msg) => write!(f, "List response error: {}", msg),
        }
    }
}
