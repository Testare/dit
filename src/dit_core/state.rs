use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    version: usize,
    hp: isize,
}

impl State {
    pub fn version(&self) -> usize {
        self.version
    }

    pub fn hp(&self) -> isize {
        self.hp
    }

    pub fn update_version(&self, version: usize) -> State {
        if self.version > version {
            panic!(
                "Attempting to upgrade from version {} to previous version {} is not allowed.",
                self.version, version
            );
        }
        State { version, ..*self }
    }
}

impl Default for State {
    fn default() -> State {
        State {
            version: 0_01_00,
            hp: 100,
        }
    }
}
