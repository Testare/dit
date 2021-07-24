use super::super::Action;
use super::{Message, Mode};
use serde_json;
use std::{fmt, io};

/// Enum for all dit-related errors
#[derive(Debug)]
pub enum Error<A: Action> {
    IoError(String, io::Error),
    IoError2(io::Error),
    SerdeError(serde_json::Error),
    FailedValidation {
        file_name: String,
        line_number: usize,
        last_message: Message<A>,
        failed_message: Message<A>,
    },
    BadAction, // Impl better later
    WrongMode {
        mode: Mode,                // Mode of the file
        expected_modes: Vec<Mode>, // Mode of the
    },
}

impl<A: Action> fmt::Display for Error<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(file_name, err) if err.kind() == io::ErrorKind::NotFound  => write!(f, "Um, sorry, but I can't find {}.", file_name.as_str()),
            Error::FailedValidation {file_name, line_number, last_message, failed_message} => write!(f,"Welp, looks like this file, {}, is invalid. You've got a bad link on line {}.\nFrom {}\nTo-> {}", file_name, line_number, last_message, failed_message),
            Error::IoError(file_name, err) => write!(f, "Sorry, I tried reading {}, but I ran into a problem and got this error:\n{}", file_name, err),
            Error::IoError2(err) => write!(f, "Sorry, I tried reading this book, but I ran into a problem and got this error:\n{}", err),
            Error::SerdeError(err) => write!(f, "Dang it, I messed up. I ran into a serialization problem:\n{}", err),
            Error::WrongMode {mode, expected_modes} => write!(f, "Mate, this file is in mode {:?}. We need it to be in one of these modes: {:?}", mode, expected_modes),
            Error::BadAction => write!(f , "This error message is a work in progress, but an action did a bad"), // TODO fix this error message
        }
    }
}
