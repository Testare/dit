use super::{dit_result, io_error, Error, Message, Action, ToActionUpdate};
use serde_json;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

pub fn with_game_state<'a, A, U, F>(file_name: &'a str, action_apply: F) -> Result<(), Error<A>>
where
    A: Action,
    F: FnOnce(&A::State) -> Result<U, Error<A>>,
    U: ToActionUpdate<A>,
{
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(file_name)
        .map_err(io_error(file_name))?;

    let (state, last_message) = BufReader::new(&file).lines().try_fold(
        (A::State::default(), Message::default()),
        |(state, _), line_result| {
            let line = line_result.map_err(io_error(file_name))?;
            let new_message =
                serde_json::from_str::<Message<A>>(line.as_str()).map_err(Error::SerdeError)?;
            Ok((new_message.action().apply(state), new_message))
        },
    )?;

    let (next_action, next_update) = action_apply(&state)?.to_action_update();
    if next_update.successful {
        let next_message = last_message.gen_next_message(next_action, &state);
        let message_string: String =
            // serde_json::to_string(&next_message).map_err(Error::SerdeError)?;
            dit_result(serde_json::to_string(&next_message))?;
        write!(file, "{}\n", message_string).map_err(io_error(file_name))?;
    }
    Ok(())
}
