use super::dit_core::{self, Action, HexString, Ledger, Mode, PendingLedger, State};
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
pub enum ActionA {
    NoOp,
    Marker { content: String },
    UpdateVersion { version: usize },
    AttemptSeekEncounter,
    AttemptLearnSpell { spell: spells::Spell },
    CastSpell { spell: spells::Spell },
}

impl Action for ActionA {
    type State = StateA;

    fn apply(
        &self,
        _ledger: &PendingLedger<ActionA>,
        state: Self::State,
    ) -> Result<Self::State, dit_core::Error<ActionA>> {
        match self {
            ActionA::UpdateVersion { version } => state.update_version(*version),
            _ => Ok(state),
        }
    }

    fn applicable(&self, _ledger: &Ledger<Self>, state: &Self::State) -> bool {
        match self {
            ActionA::UpdateVersion { version } => state.version() < *version,
            _ => true,
        }
    }

    /// The bit cost of the action.
    ///
    /// In order for an action to be applied and saved, it takes a certain
    /// amount of computational work as a cost. An action is wrapped in a
    /// [`Message`](super::Message) object with a hexadecimal key. When a new
    /// action is being applied, to be saved it needs to be added to a message
    /// with a randomly generated key. The hash of the previous message's key,
    /// the action json, and the randomly generated key is compared with the
    /// previous message's key. The last `n` bits have to match.
    ///
    /// This function determines how many bits have to match, which determines
    /// how long on average it will take to apply an action. It takes state as
    /// a parameter, since the state might influence how difficult an action
    /// would be. For instance, a high level wizard might learn spells easier
    /// than a warrior, or you might be able to store mana to cast a spell later
    /// for cheaper.
    fn bit_cost(&self, _state: &Self::State) -> usize {
        match self {
            ActionA::UpdateVersion { .. } => 1,
            ActionA::AttemptSeekEncounter => 5,
            ActionA::CastSpell { .. } => 8,
            _ => 5,
        }
    }
}

impl Default for ActionA {
    fn default() -> Self {
        ActionA::NoOp
    }
}

impl ToString for ActionA {
    fn to_string(&self) -> String {
        serde_json::to_string(self).expect("All actions should be serializable")
    }
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

    pub fn update_version(&self, version: usize) -> Result<StateA, dit_core::Error<ActionA>> {
        if self.version > version {
            Err(dit_core::Error::BadAction)
        } else {
            Ok(StateA { version, ..*self })
        }
    }
}

impl State for StateA {
    fn read_header_line(self, _header_line: &str) -> Self {
        self.clone()
    }

    fn root_hash(&self) -> HexString {
        HexString::default()
    }

    fn mode() -> Mode {
        Mode::A
    }
}

impl Default for StateA {
    fn default() -> StateA {
        StateA {
            version: 0_01_00,
            hp: 100,
        }
    }
}
