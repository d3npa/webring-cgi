use qstring::QString;
use std::env;
use std::str::FromStr;

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

fn main() -> Result<(), anyhow::Error> {
    /* ?before=:domain ; ?after=:domain ; ?random_from=:domain */
    let query = env::var("QUERY_STRING")
        .unwrap_or_else(|_| http::internal_server_error("Error: Environment variable not found"));

    let qstring = QString::from(query.as_str());

    let pairs = qstring.to_pairs();
    let (command, site) = match pairs.first() {
        Some(p) => p,
        None => http::html_text_response("200 OK", INFO),
    };

    let command = match Command::from_str(command) {
        Ok(command) => command,
        Err(_) => http::bad_request(USAGE),
    };

    let list = reqwest::blocking::get(LIST_URL)?.text()?;
    let webring = Webring::new(&list);

    let result = match command {
        Command::BEFORE => webring.before(site),
        Command::AFTER => webring.after(site),
        Command::RANDOM => webring.random(site),
        Command::LIST => http::plain_text_response("200 OK", &list),
    };

    match result {
        Some(to_result) => http::redirect(to_result),
        None => http::not_found("No result found"),
    }

    Ok(())
}
