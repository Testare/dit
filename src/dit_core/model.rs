mod action_interface;
mod book;
mod error;
mod hex_string;
mod ledger;
mod message;

pub use action_interface::ActionInterface;
pub use book::Book;
pub use error::Error;
pub use hex_string::HexString;
pub use ledger::Ledger;
pub use message::Message;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum Mode {
    // Should this be an enum, or just a newtape for Char? Opens possibility for many unicode characters instead of just whatever is defined here, allowing for unofficial modes
    A,
    B,
}

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
}