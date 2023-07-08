use thiserror::Error;

#[derive(Debug, Error)]
pub enum CgiError {
    #[error("Environment Not Found")]
    EnvNotFound,
    #[error("Malformed Query")]
    MalformedQuery,
}

#[derive(Copy, Clone, Debug, Error)]
pub enum WebringError {
    #[error("Error Downloading List")]
    DownloadingList,
    #[error("Error Parsing URL")]
    ParsingUrl,
    #[error("Unknown Command. Valid commands: before | after | random | list")]
    UnknownCommand,
    #[error("No Result Found")]
    NotFound,
}

impl From<reqwest::Error> for WebringError {
    fn from(_: reqwest::Error) -> Self {
        Self::DownloadingList
    }
}

impl From<url::ParseError> for WebringError {
    fn from(_: url::ParseError) -> Self {
        Self::ParsingUrl
    }
}
