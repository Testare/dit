use super::{Action, Error, Message};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

/// Should be renamed to something like "apply_action_to_file",
/// though it would probably be worth seperating into sub functions,
/// "reader_to_state" : Reader/File -> Result<State> (Have to figure out how to get the typing right)
/// "apply_action_to_state" : State -> Action -> Result<(State, Message)>
/// "save_message" : Writer/File  -> Message -> Result<()>
///
/// Takes a filename, and a clojure that generates an Action (Or Error). If clojure returns successful,
/// We attempt to apply it to the state, and if THAT works, we save it to the file.
pub fn with_game_state<A, F>(file_name: &str, action_apply: F) -> Result<(), Error<A>>
where
    A: Action,
    F: FnOnce(&A::State) -> Result<A, Error<A>>,
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

    let next_action = action_apply(&state)?;
    let next_message = last_message.gen_next_message(next_action, &state);
    let message_string: String = dit_result(serde_json::to_string(&next_message))?;
    write!(file, "{}\n", message_string).map_err(io_error(file_name))?;
    Ok(())
}

/// Checks whether a file is valid by checking the hashes of the Messages
/// It also fully constructs the game state in the process, since we sometimes
/// need that state to determine the number of bits that need to match.
pub fn validate<A: Action>(file_name: &str) -> Result<(), Error<A>> {
    let file = File::open(file_name).map_err(io_error(file_name))?;
    let lines = BufReader::new(file).lines();
    lines
        .map(|line_result| line_result.unwrap())
        .map(|line| serde_json::from_str::<Message<A>>(line.as_str()).unwrap()) // Should probably refactor to return a SeDe error
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

/// Allows use of .map_err(io_error(file_name)) when an error occurs to make a
/// [`std::io::Result`] into a dit result. Might reimplement differently later.
pub fn io_error<A: Action>(file_name: &str) -> impl FnOnce(io::Error) -> Error<A> {
    let file_name_owned = String::from(file_name);
    move |error| Error::IoError(file_name_owned, error)
}

/// TODO Make this use a trait to be a bit more generic
pub fn dit_result<A: Action, T>(result: Result<T, serde_json::Error>) -> Result<T, Error<A>> {
    result.map_err(Error::SerdeError)
}


#[cfg(test)]
mod test {

    #[test]
    #[ignore]
    fn test_validate() {
        unimplemented!("Currently not sure how to do this as validate takes a filename as input. Will probably need to refactor")
    }

    #[test]
    #[ignore]
    fn test_with_game_state_clojure_ends_with_result_of_action() {
        unimplemented!("Currently not sure how to do this as game state takes a filename as input. Will probably need to refactor.")
    }

    #[test]
    #[ignore]
    fn test_with_game_state_clojure_ends_with_result_of_action_and_update() {
        unimplemented!("Currently not sure how to do this as game state takes a filename as input. Will probably need to refactor.")
        // Also might need to change this implementation: The Update is beter as an action onto state output than the clojure that produces an action.
        // It might also be better implemented with an Option in both cases
    }
}
