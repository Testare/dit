use super::state::{update, State, Update};
use serde::{Deserialize, Serialize};

pub mod spells {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum Spell {
        FireBall,
        IceDagger,
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Action {
    NoOp,
    Marker(String),
    Version(usize),
    AttemptSeekEncounter,
    AttemptLearnSpell(spells::Spell),
    CastSpell(spells::Spell),
}

impl Action {
    pub fn execute(&self, state: &State) -> (State, Update) {
        match self {
            Action::Version(version) => (
                State {
                    version: *version,
                    ..*state
                },
                update(format!("Updated version to {}", version)),
            ),
            _ => (state.clone(), Update::default()),
        }
    }

    pub fn bit_cost(&self, _state: &State) -> usize {
        match self {
            Action::Version(_) => 1,
            Action::AttemptSeekEncounter => 5,
            Action::CastSpell(_) => 8,
            _ => 5,
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::NoOp
    }
}

impl ToString for Action {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("All actions should be serializable")
    }
}
