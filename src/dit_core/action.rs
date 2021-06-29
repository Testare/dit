use super::state::{State, StateA};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub mod spells {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub enum Spell {
        FireBall,
        IceDagger,
    }
}

pub trait Action: ToString + Serialize + DeserializeOwned + Default {
    type State: State;
    fn apply(&self, state: Self::State) -> Self::State ;
    fn bit_cost(&self, state: &Self::State) -> usize;
}

/// Represents a change in state.
/// 
/// Must be careful about extending: We want to make sure that old versions of 
/// actions always serialize in code, since we need to always be able to maintain
/// old specs in order to validate current games.
/// 
/// Similar conceptually to actions/reducers in [Redux](https://redux.js.org)
/// 
/// ActionA is responsible for the logic to change the state (In regards to 
/// version), and determining bit cost from current state. It is not responsible
/// for user interaction of any sort for generating the action.
/// 
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
    /// Applies the change represented by the action to the state.
    /// TODO consider using mutable borrow instead of move-and-return
    fn apply(&self, state: Self::State) -> Self::State {
        match self {
            ActionA::UpdateVersion { version } => state.update_version(*version),
            _ => state,
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
