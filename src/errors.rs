use reqwest::Error as ReqwestError;
use url::ParseError as UrlParserError;

#[derive(Debug)]
#[repr(u16)]
#[non_exhaustive]
pub enum Error {
    InvalidUrl(UrlParserError) = 10,
    Http(ReqwestError) = 100,
}

impl From<UrlParserError> for Error {
    fn from(value: UrlParserError) -> Self {
        Self::InvalidUrl(value)
    }
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        Self::Http(value)
    }
}
