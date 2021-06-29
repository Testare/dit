use super::{io_error, DitAction, DitState, Error, Message, State};
use serde_json;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn validate<A: DitAction> (file_name: &str) -> Result<(), Error<A>> {
    let file = File::open(file_name).map_err(io_error(file_name))?;
    let lines= BufReader::new(file).lines();
    lines
        .map(|line_result| line_result.unwrap())
        .map(|line| serde_json::from_str::<Message<A>>(line.as_str()).unwrap())
        .zip(1..)
        .try_fold(
            (A::State::default(), Message::default()),
            |(state, last_message), (next_message, line_number)| {
                if last_message.accepts_next_message(&next_message, &state) {
                    Ok((next_message.action().apply(state), next_message))
                } else {
                    Err(Error::FailedValidation {
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
