use super::{Error, HexString, Ledger, Mode, PendingLedger};
use serde::{de::DeserializeOwned, Serialize};

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
/// for user interaction of any sort for generating the action. It is responsible
/// for determining whether it CAN be applied to state.
pub trait Action: ToString + Serialize + DeserializeOwned + Default + Clone {
    type State: State;
    // fn apply(&self, state: Self::State) -> Self::State; //Option<Self::State> or Result<<Self::State, Error> ?
    fn apply(
        &self,
        ledger: &PendingLedger<Self>,
        state: Self::State,
    ) -> Result<Self::State, Error<Self>>;
    fn applicable(&self, ledger: &Ledger<Self>, state: &Self::State) -> bool; // Should perhaps return an Option<Error<Self>> instead?
    fn bit_cost(&self, state: &Self::State) -> usize;
}

pub trait State: Default {
    /// Read state from header lines of a file
    fn read_header_line(self, header_line: &str) -> Self;
    /// A hash that is supposedly unique to the file.
    fn root_hash(&self) -> HexString;
    /// What Mode this state is for
    fn mode() -> Mode;
}
