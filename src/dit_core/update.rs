use serde::{Deserialize, Serialize};

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
