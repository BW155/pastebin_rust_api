use std::{io, result};

use reqwest;
use reqwest::StatusCode;

pub type Result<T> = result::Result<T, Error>;

/// The Errors which may occur when using the Pastebin Rust API.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    HttpError(reqwest::Error),
    SerializeError(reqwest::Error),
    RequestError(reqwest::Error),
    /// Contains the status code of the request
    RequestFailed(StatusCode),
    PasteBinError(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        match err {
            reqwest::Error::Http(..) => Error::HttpError(err),
            reqwest::Error::Serialize(..) => Error::SerializeError(err),
            _ => Error::RequestError(err),
        }
    }
}

pub fn check_for_error(result: String) -> Result<String> {
    if result.starts_with("Bad API request") || result.starts_with("Post limit") {
        Err(Error::PasteBinError(result))
    } else {
        Ok(result)
    }
}
