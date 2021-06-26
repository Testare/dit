use super::Action;
use serde::{Deserialize, Serialize};

pub trait ToActionUpdate {
    fn to_action_update(self) -> (Action, Update);
}

impl ToActionUpdate for (Action, Update) {
    fn to_action_update(self) -> (Action, Update) {
        self
    }
}

impl ToActionUpdate for Action {
    fn to_action_update(self) -> (Action, Update) {
        (self, Update::default())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    pub message: String,
    pub successful: bool,
}

impl Default for Update {
    fn default() -> Update {
        Update {
            message: String::new(),
            successful: true,
        }
    }
}
