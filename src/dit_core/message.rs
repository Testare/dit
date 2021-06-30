use super::{work::bit_match, Action};

use hex;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use sha3::{Digest, Sha3_224};
use std::fmt::{self, Display};
use std::iter;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "A: Action")]
pub struct Message<A: Action> {
    key: HexString,
    action: A,
}

impl <A: Action> Message<A> {

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

impl <A: Action> Display for Message<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        serde_json::to_string(self).map_or_else(|_| Err(fmt::Error), |json| write!(f, "{}", json))
    }
}

impl <A: Action> Default for Message<A> {
    fn default() -> Self {
        Message {
            key: HexString::default(),
            action: A::default(),
        }
    }
}

/// A wrapper for a string of hexadecimal characters
/// There is no way to initialize it with a string of other characters other than deserializing tampered data, so it should be safe to deserialize
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct HexString(String);

impl HexString {
    pub fn to_bytes(&self) -> Vec<u8> {
        Vec::from(self)
    }
}

impl From<&[u8]> for HexString {
    fn from(bytes:&[u8]) -> HexString {
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