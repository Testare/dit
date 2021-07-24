use super::super::bit_match;
use super::super::Action;
use super::HexString;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use sha3::{Digest, Sha3_224};
use std::{fmt, fmt::Display, iter};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(bound = "A: Action", from = "(HexString, A)", into = "(HexString, A)")]
pub struct Message<A: Action> {
    key: HexString,
    action: A,
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

    pub fn key(&self) -> &HexString {
        &self.key
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
    pub fn gen_next_message_with_hook<F: Fn(HexString) -> ()>(
        &self,
        action: A,
        state: &A::State,
        hook: F,
        hook_frequency: u32,
    ) -> Self {
        let hasher = self.get_hasher_for_payload(&action);
        let threshold = action.bit_cost(state);
        let prev_hash_bytes = self.key.to_bytes();
        let mut rng = thread_rng(); // Might need to pass to the function to enable reproducible testing
        let (key, _) = iter::repeat_with(|| rng.gen::<u32>())
            .map(|n| n.to_le_bytes())
            .zip((0..hook_frequency).cycle())
            .skip_while(|(key, cycle_count)| {
                let mut key_hasher = hasher.clone();
                key_hasher.update(key);
                // Might want some logic for breaking out of the program here.
                let hash = key_hasher.finalize();
                if *cycle_count == 0 {
                    hook(HexString::from(&hash[..]));
                }
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
