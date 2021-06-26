use super::action::Action;
use super::state::State;
use super::work::bit_match;

use hex;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use sha3::{Digest, Sha3_224};
use std::fmt;
use std::fmt::Display;
use std::iter;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Message {
    key: String,
    action: Action,
}

impl Message {
    pub fn action(&self) -> &Action {
        &self.action
    }

    fn get_hasher_for_payload(&self, action: &Action) -> Sha3_224 {
        let mut hasher = Sha3_224::new();
        hasher.update(self.key.as_str());
        hasher.update(action.to_string());
        hasher
    }

    pub fn accepts_next_message(&self, next_message: &Message, state: &State) -> bool {
        let mut hasher = self.get_hasher_for_payload(&next_message.action);
        hasher.update(hex::decode(&next_message.key).unwrap()); // Wrap in result?
        let threshold = next_message.action.bit_cost(state);
        bit_match(
            threshold,
            &hex::decode(self.key.as_str()).unwrap(),
            &hasher.finalize(),
        )
    }

    pub fn gen_next_message(&self, action: Action, state: &State) -> Self {
        let hasher = self.get_hasher_for_payload(&action);
        let threshold = action.bit_cost(state);
        let prev_hash_bytes =
            hex::decode(&self.key).expect("Key in message not decodeable as hex string"); // Maybe don't unwrap?
        let mut rng = thread_rng();

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
            key: hex::encode(key),
            action,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        serde_json::to_string(self).map_or_else(|_| Err(fmt::Error), |json| write!(f, "{}", json))
    }
}

impl Default for Message {
    fn default() -> Self {
        Message {
            key: String::from("00000000"),
            action: Action::default(),
        }
    }
}
