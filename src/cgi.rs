use crate::errors::CgiError;
use qstring::QString;
use std::env;

pub fn read_query() -> Result<(String, String), CgiError> {
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
