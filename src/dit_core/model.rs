use super::{bit_match, Action};

use hex;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use sha3::{Digest, Sha3_224};
use std::{fmt, fmt::Display, io, iter};

/// Enum for all dit-related errors
#[derive(Debug)]
pub enum Error<A: Action> {
    IoError(String, io::Error),
    SerdeError(serde_json::Error),
    FailedValidation {
        file_name: String,
        line_number: usize,
        last_message: Message<A>,
        failed_message: Message<A>,
    },
    BadAction, // Impl better later
}

/// A wrapper for a string of hexadecimal characters
/// There is no way to initialize it with a string of other characters other than deserializing tampered data, so it should be safe to deserialize
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HexString(String);

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "A: Action", from = "(HexString, A)", into = "(HexString, A)")]
pub struct Message<A: Action> {
    key: HexString,
    action: A,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Mode {
    A,
    B,
}

pub struct Ledger<A: Action>(Vec<Message<A>>);

impl<A: Action> Ledger<A> {
    pub fn new() -> Self {
        Ledger(vec![])
    }
}

/// Deserialize it from a tuple
impl<A: Action> From<(HexString, A)> for Message<A> {
    fn from((key, action): (HexString, A)) -> Message<A> {
        Message { key, action }
    }
}

/// Serialize it from a tuple
impl<A: Action> Into<(HexString, A)> for Message<A> {
    fn into(self) -> (HexString, A) {
        (self.key, self.action)
    }
}

impl<A: Action> Message<A> {
    /// Returns the action this message represents
    pub fn action(&self) -> &A {
        &self.action
    }

    fn get_hasher_for_payload(&self, action: &A) -> Sha3_224 {
        let mut hasher = Sha3_224::new();
        hasher.update(self.key.to_bytes());
        hasher.update(serde_json::to_string(action).expect("Issue serializing action"));
        hasher
    }

    /// Checks whether this message and the next message are validly linked.
    ///
    /// The state is necessary as we might need that to determine the bit cost
    /// for an action.
    pub fn accepts_next_message(&self, next_message: &Message<A>, state: &A::State) -> bool {
        let mut hasher = self.get_hasher_for_payload(&next_message.action);
        hasher.update(&next_message.key.to_bytes());
        let threshold = next_message.action.bit_cost(state);
        bit_match(
            threshold,
            Vec::from(&self.key).as_slice(),
            &hasher.finalize(),
        )
    }

    /// Generate a message that can follow this one for the specified action.
    pub fn gen_next_message(&self, action: A, state: &A::State) -> Self {
        let hasher = self.get_hasher_for_payload(&action);
        let threshold = action.bit_cost(state);
        let prev_hash_bytes = self.key.to_bytes();
        let mut rng = thread_rng(); // Might need to pass to the function to enable reproducible testing

        let key = iter::repeat_with(|| rng.gen::<u32>())
            .map(|n| n.to_le_bytes())
            .skip_while(|key| {
                let mut key_hasher = hasher.clone();
                key_hasher.update(key);
                // Might want some logic for breaking out of the program here.
                let hash = key_hasher.finalize();
                !bit_match(threshold, &prev_hash_bytes, &hash)
            })
            .next()
            .unwrap();

        // Convert previous hash to bytes
        // Sha digest the message.
        // Generate random hashes. If we use a u32, we can make hashes of length 32 bits. Would we ever want one with more?
        // Take the random hashes until one matches with threshold amount of bits at the end.
        Message {
            key: HexString::from(&key[..]),
            action,
        }
    }
}

impl<A: Action> Display for Message<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        serde_json::to_string(self).map_or_else(|_| Err(fmt::Error), |json| write!(f, "{}", json))
    }
}

impl<A: Action> Default for Message<A> {
    fn default() -> Self {
        Message {
            key: HexString::default(),
            action: A::default(),
        }
    }
}

impl HexString {
    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::from(self)
    }
}

impl From<&[u8]> for HexString {
    fn from(bytes: &[u8]) -> HexString {
        HexString(hex::encode(bytes))
    }
}

impl From<&HexString> for Vec<u8> {
    fn from(hex_string: &HexString) -> Vec<u8> {
        hex::decode(&hex_string.0).unwrap()
    }
}

impl ToString for HexString {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Default for HexString {
    fn default() -> Self {
        HexString(String::from("00000000"))
    }
}

impl<A: Action> fmt::Display for Error<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IoError(file_name, err) if err.kind() == io::ErrorKind::NotFound  => write!(f, "Um, sorry, but I can't find {}.", file_name.as_str()),
            Error::FailedValidation {file_name, line_number, last_message, failed_message} => write!(f,"Welp, looks like this file, {}, is invalid. You've got a bad link on line {}.\nFrom {}\nTo-> {}", file_name, line_number, last_message, failed_message),
            Error::IoError(file_name, err) => write!(f, "Sorry, I tried reading {}, but I ran into a problem and got this error:\n{}", file_name, err),
            Error::SerdeError(err) => write!(f, "Dang it, I messed up. I ran into a serialization problem:\n{}", err),
            Error::BadAction => write!(f , "This error message is a work in progress, but an action did a bad") // TODO fix this error message
        }
    }
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

/// WIP not sure quite how to do this.
/// A struct that can be passed with the execution of an action to have certain things happen
struct ActionHooks {
    invalid: Box<dyn FnOnce() -> ()>,
    iter: Box<dyn Fn(HexString) -> ()>,
    success: Box<dyn Fn(HexString) -> ()>,
    iter_frequency: u32,
}

impl Default for ActionHooks {
    fn default() -> Self {
        ActionHooks {
            iter_frequency: 1,
            invalid: Box::new(|| {}),
            iter: Box::new(|_| {}),
            success: Box::new(|_| {}),
        }
    }
}
