use serde::{Deserialize, Serialize};

pub trait ToActionUpdate<A> {
    fn to_action_update(self) -> (A, Update);
}

impl <A> ToActionUpdate<A> for (A, Update) {
    fn to_action_update(self) -> (A, Update) {
        self
    }
}

impl <A> ToActionUpdate<A> for A {
    fn to_action_update(self) -> (A, Update) {
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
