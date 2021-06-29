use super::{Message, DitAction, DitState};
use serde_json;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error<A: DitAction> {
    IoError(String, io::Error),
    SerdeError(serde_json::Error),
    FailedValidation {
        file_name: String,
        line_number: usize,
        last_message: Message<A>,
        failed_message: Message<A>,
    },
}

impl <A: DitAction> fmt::Display for Error<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(file_name, err) if err.kind() == io::ErrorKind::NotFound  => write!(f, "Um, sorry, but I can't find {}.", file_name.as_str()),
            Error::FailedValidation {file_name, line_number, last_message, failed_message} => write!(f,"Welp, looks like this file, {}, is invalid. You've got a bad link on line {}.\nFrom {}\nTo-> {}", file_name, line_number, last_message, failed_message),
            Error::IoError(file_name, err) => write!(f, "Sorry, I tried reading {}, but I ran into a problem and got this error:\n{}", file_name, err),
            Error::SerdeError(err) => write!(f, "Dang it, I messed up. I ran into a serialization problem:\n{}", err)
        }
    }
}

pub fn  io_error<A: DitAction>(file_name: &str) -> impl FnOnce(io::Error) -> Error<A> {
    let file_name_owned = String::from(file_name);
    move |error| Error::IoError(file_name_owned, error)
}

pub fn dit_result<A: DitAction, T>(result: Result<T, serde_json::Error>) -> Result<T, Error<A>> {
    result.map_err(Error::SerdeError)
}
