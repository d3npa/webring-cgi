use qstring::QString;
use std::env;
use std::str::FromStr;
use url::Url;

use webring_cgi::{http, webring::Webring};

const LIST_URL: &str = "https://raw.githubusercontent.com/VVX7/haunted-webring/main/webring.txt";
const INFO: &str = include_str!("includes/info.html");
const USAGE: &str = "Must pass exactly one parameter: before | after | random | list";

#[derive(Copy, Clone, Debug, PartialEq)]
struct UnknownCommandError;

enum Command {
    BEFORE,
    AFTER,
    RANDOM,
    LIST,
}

impl FromStr for Command {
    type Err = UnknownCommandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "before" => Ok(Command::BEFORE),
            "after" => Ok(Command::AFTER),
            "random" => Ok(Command::RANDOM),
            "list" => Ok(Command::LIST),
            _ => Err(UnknownCommandError {}),
        }
    }
}

fn get_first_query_or_error() -> (String, String) {
    let query = env::var("QUERY_STRING")
        .unwrap_or_else(|_| http::internal_server_error("Error: Environment variable not found"));

    let qstring = QString::from(query.as_str());
    let pairs = qstring.to_pairs();

    match pairs.first() {
        Some(p) => (p.0.to_owned(), p.1.to_owned()),
        None => http::html_text_response("200 OK", INFO),
    }
}

fn match_command_or_show_usage(command: &str) -> Command {
    match Command::from_str(command) {
        Ok(command) => command,
        Err(_) => http::bad_request(USAGE),
    }
}

fn load_remote_webring() -> Result<Webring, anyhow::Error> {
    let list = reqwest::blocking::get(LIST_URL)?.text()?;
    Ok(Webring::new(&list))
}

fn main() -> Result<(), anyhow::Error> {
    let (command, site) = get_first_query_or_error();
    let command = match_command_or_show_usage(&command);
    let webring = load_remote_webring()?;

    let result = match command {
        Command::BEFORE => webring.before(&Url::parse(&site)?),
        Command::AFTER => webring.after(&Url::parse(&site)?),
        Command::RANDOM => webring.random(&Url::parse(&site)?),
        Command::LIST => http::redirect(LIST_URL),
    };

    match result {
        Some(to_result) => http::redirect(&to_result.to_string()),
        None => http::not_found("No result found"),
    }
}
