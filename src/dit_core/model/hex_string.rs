use serde::{Deserialize, Serialize};

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