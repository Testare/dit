use serde::{Deserialize, Serialize};

pub trait State: Default {
    // type Action: ToString + Serialize;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StateA {
    version: usize,
    hp: isize,
}

impl StateA {
    pub fn version(&self) -> usize {
        self.version
    }

    pub fn hp(&self) -> isize {
        self.hp
    }

    pub fn update_version(&self, version: usize) -> StateA {
        if self.version > version {
            panic!(
                "Attempting to upgrade from version {} to previous version {} is not allowed.",
                self.version, version
            );
        }
        StateA { version, ..*self }
    }
}

impl State for StateA {

}

impl Default for StateA {
    fn default() -> StateA {
        StateA {
            version: 0_01_00,
            hp: 100,
        }
    }
}
