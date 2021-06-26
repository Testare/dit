use super::dit_core::message::Message;
use super::dit_core::state::State;
use serde_json;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub enum ValidationError {
    IoError(String, io::Error),
    FailedValidation {
        file_name: String,
        line_number: usize,
        last_message: Message,
        failed_message: Message,
    },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::IoError(file_name, err) if err.kind() == io::ErrorKind::NotFound  => write!(f, "Um, sorry, but I can't find {}.", file_name.as_str()),
            ValidationError::FailedValidation {file_name, line_number, last_message, failed_message} => write!(f,"Welp, looks like this file, {}, is invalid. You've got a bad link on line {}.\nFrom {}\nTo-> {}", file_name, line_number, last_message, failed_message),
            ValidationError::IoError(file_name, err) => write!(f, "Sorry, I tried to validate {}, but I ran into a problem and got this error:\n{}", file_name, err)
        }
    }
}

pub fn validate(file_name: &str) -> Result<(), ValidationError> {
    // let contents: String = fs::read_to_string(file_name)?;
    // if let Ok(file) = File::open(file_name) {
    let file = File::open(file_name)
        .map_err(|err| ValidationError::IoError(String::from(file_name), err))?;
    let lines = BufReader::new(file).lines();
    lines
        .map(|line_result| line_result.unwrap())
        .map(|line| serde_json::from_str::<Message>(line.as_str()).unwrap())
        .zip(1..)
        .try_fold(
            (State::default(), Message::default()),
            |(state, last_message), (next_message, line_number)| {
                if last_message.accepts_next_message(&next_message, &state) {
                    Ok((next_message.action().apply(state), next_message))
                } else {
                    Err(ValidationError::FailedValidation {
                        file_name: String::from(file_name),
                        failed_message: next_message,
                        last_message,
                        line_number,
                    })
                }
            },
        )
        .and(Ok(())) // We don't want to pass final value of the fold on
}
