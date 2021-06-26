use super::action::Action;
use super::message::Message;
use super::state::{State, Update};
use serde_json;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    SerdeError(serde_json::Error),
}

pub trait ToActionUpdate {
    fn to_action_update(self) -> (Action, Update);
}

impl ToActionUpdate for (Action, Update) {
    fn to_action_update(self) -> (Action, Update) {
        self
    }
}

impl ToActionUpdate for Action {
    fn to_action_update(self) -> (Action, Update) {
        (self, Update::default())
    }
}

pub fn with_game_state<A, F, W>(
    file_name: &str,
    writer: &mut W,
    action_apply: F,
) -> Result<(), Error>
where
    F: FnOnce(&State, &mut W) -> Result<A, Error>,
    A: ToActionUpdate,
    W: io::Write,
{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(file_name)
        .map_err(Error::IoError)?;

    let (state, last_message) = BufReader::new(&file).lines().try_fold(
        (State::default(), Message::default()),
        |(state, _), line_result| {
            let line = line_result.map_err(Error::IoError)?;
            let new_message =
                serde_json::from_str::<Message>(line.as_str()).map_err(Error::SerdeError)?;
            Ok((new_message.action().apply(state), new_message))
        },
    )?;

    let (next_action, next_update) = action_apply(&state, writer)?.to_action_update();
    if next_update.successful {
        let next_message = last_message.gen_next_message(next_action, &state);
        let message_string: String =
            serde_json::to_string(&next_message).map_err(Error::SerdeError)?;
        write!(file, "{}\n", message_string).map_err(Error::IoError)?;
    }
    Ok(())
}
