use super::state::State;
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
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Action {
    NoOp,
    Marker { content: String },
    UpdateVersion { version: usize },
    AttemptSeekEncounter,
    AttemptLearnSpell { spell: spells::Spell },
    CastSpell { spell: spells::Spell },
}

impl Action {
    // apply (Does not handle user interaction, just the result. What is used to process change in state from message in file
    pub fn apply(&self, state: State) -> State {
        match self {
            Action::UpdateVersion { version } => state.update_version(*version),
            _ => state,
        }
    }

    pub fn bit_cost(&self, _state: &State) -> usize {
        match self {
            Action::UpdateVersion { .. } => 1,
            Action::AttemptSeekEncounter => 5,
            Action::CastSpell { .. } => 8,
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
