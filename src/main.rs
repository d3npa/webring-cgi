use std::str::FromStr;
use url::Url;
use webring_cgi::errors::{CgiError, WebringError};
use webring_cgi::{cgi, http, Webring};

const LIST_URL: &str = "https://raw.githubusercontent.com/VVX7/haunted-webring/main/webring.txt";
const INFO: &str = include_str!("includes/info.html");

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

fn process_command(command: &str, site: &str) -> Result<String, WebringError> {
    let command = Command::from_str(command)?;

    // process List commands before attempting to load remote webring
    if command == Command::List {
        return Ok(LIST_URL.to_owned());
    }

    let webring = Webring::load_remote(LIST_URL)?;

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
    let (command, site) = match cgi::read_query() {
        Ok((c, s)) => (c, s),
        Err(CgiError::MalformedQuery) => http::html_text_response("200 OK", INFO),
        Err(e) /* CgiError::EnvNotFound */ => http::internal_server_error(&e.to_string()),
    };

    match process_command(&command, &site) {
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
