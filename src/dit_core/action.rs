use super::State;
use serde::{Serialize, de::DeserializeOwned};
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
pub trait Action: ToString + Serialize + DeserializeOwned + Default {
    type State: State;
    fn apply(&self, state: Self::State) -> Self::State ;
    fn bit_cost(&self, state: &Self::State) -> usize;
}

