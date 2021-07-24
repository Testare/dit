//! This module is for the core of the `dit` game, which should not change even
//! as the interface changes or has different interfaces.

mod model;
mod ops;
mod traits;
mod work;

pub use model::{Error, HexString, PendingLedger, Ledger, Message, Mode, ActionInterface};
pub use ops::{validate, with_game_state, read_state};
pub use traits::{Action, State};

use work::bit_match;

#[cfg(test)]
mod test {
    use super::HexString;
    use rand::{thread_rng, Rng};

    #[test]
    fn hex_string_from_bytes_to_bytes_is_equal() {
        for _ in 0..1000 {
            let random_n = thread_rng().gen::<u32>();
            let hex_string = HexString::from(&random_n.to_le_bytes()[..]);
            let mut hex_bytes: [u8; 4] = [0; 4];
            for (i, byte) in (0..).zip(hex_string.to_bytes().into_iter()) {
                hex_bytes[i] = byte;
            }
            let converted = u32::from_le_bytes(hex_bytes);
            println!("{} = {}", random_n, converted);
            assert_eq!(random_n, converted)
        }
    }

    #[test]
    #[ignore]
    fn created_message_is_accepted_by_previous_message() {
        todo!();
    }

    #[test]
    #[ignore]
    fn sede_test_messsage() {
        todo!();
    }

    #[test]
    #[ignore]
    fn tampered_message_is_not_accepted_by_previous_message() {
        todo!("Create a message, serialize it to string, replace the content with string replace (Use a non hex character), then deserialize it and see that it doesn't validate. Test with assert!(!..), not should_panic");
    }

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
