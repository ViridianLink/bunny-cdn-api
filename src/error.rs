pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidRegion(String),
    InvalidPath(std::path::PathBuf),
    ListResponseError(String),

    Io(std::io::Error),
    Reqwest(reqwest::Error),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidRegion(region) => write!(f, "Invalid region: {}", region),
            Error::InvalidPath(path) => write!(f, "Invalid path: {:?}", path),
            Error::ListResponseError(msg) => write!(f, "List response error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            Error::InvalidHeaderValue(err) => write!(f, "Invalid header value: {}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(error: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(error)
    }
}
