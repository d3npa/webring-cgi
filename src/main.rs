use qstring::QString;
use std::env;
use std::str::FromStr;
use url::Url;

use webring_cgi::{http, webring::Webring};

const LIST_URL: &str = "https://raw.githubusercontent.com/VVX7/haunted-webring/main/webring.txt";
const INFO: &str = include_str!("includes/info.html");

#[derive(Debug, thiserror::Error)]
enum CgiError {
    #[error("Environment Not Found")]
    EnvNotFound,
    #[error("Malformed Query")]
    MalformedQuery,
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
enum WebringError {
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

#[derive(PartialEq)]
enum Command {
    Before,
    After,
    Random,
    List,
}

impl FromStr for Command {
    type Err = WebringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "before" => Ok(Command::Before),
            "after" => Ok(Command::After),
            "random" => Ok(Command::Random),
            "list" => Ok(Command::List),
            _ => Err(WebringError::UnknownCommand),
        }
    }
}

fn load_remote_webring() -> Result<Webring, WebringError> {
    let list = reqwest::blocking::get(LIST_URL)?.text()?;
    Ok(Webring::new(&list))
}

fn read_cgi_query() -> Result<(String, String), CgiError> {
    let query = match env::var("QUERY_STRING") {
        Ok(v) => v,
        Err(_) => return Err(CgiError::EnvNotFound),
    };

    let query_string = QString::from(query.as_str());
    let pairs = query_string.to_pairs();

    match pairs.first() {
        Some(p) => Ok((p.0.to_owned(), p.1.to_owned())),
        None => Err(CgiError::MalformedQuery),
    }
}

fn process_webring_command(command: &str, site: &str) -> Result<String, WebringError> {
    let command = Command::from_str(command)?;

    // process List commands before attempting to load remote webring
    if command == Command::List {
        return Ok(LIST_URL.to_owned());
    }

    let webring = load_remote_webring()?;

    let url = Url::parse(site)?;

    match command {
        Command::Before => webring.before(&url),
        Command::After => webring.after(&url),
        Command::Random => webring.random(&url),
        Command::List => panic!("shouldn't be here :S"),
    }
    .map(url::Url::to_string)
    .ok_or(WebringError::NotFound)
}

fn main() {
    let (command, site) = match read_cgi_query() {
        Ok((c, s)) => (c, s),
        Err(CgiError::MalformedQuery) => http::html_text_response("200 OK", INFO),
        Err(e) /* CgiError::EnvNotFound */ => http::internal_server_error(&e.to_string()),
    };

    match process_webring_command(&command, &site) {
        Ok(to) => http::redirect(&to),
        Err(e) => {
            use WebringError::*;
            match e {
                DownloadingList | ParsingUrl => http::internal_server_error(&e.to_string()),
                UnknownCommand => http::bad_request(&e.to_string()),
                NotFound => http::not_found(&e.to_string()),
            }
        }
    }
}
