use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub version: usize,
    pub hp: isize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Update {
    pub message: String,
    pub successful: bool,
}

pub fn update<S: ToString>(message: S) -> Update {
    Update {
        message: message.to_string(),
        successful: true,
    }
}

impl Default for Update {
    fn default() -> Update {
        Update {
            message: String::new(),
            successful: true,
        }
    }
}

/* use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
struct PlayerState {
    house: Vec<Vec<i32>>,
    version: PlayerVersioning,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum PlayerVersioning {
    V0,
}

trait QitSystemState {
    type Version;
    type Message;

    fn get_version(&self) -> Self::Version;

    fn update_to_at_least_version(&self, version: Self::Version) -> Self;

    fn initial_state() -> Self;

    fn apply_message(&self, message: Self::Message) -> Self;
}

impl QitSystemState for PlayerState {
    type Version = PlayerVersioning;
    type Message = PlayerMessage;

    fn get_version(&self) -> PlayerVersioning {
        PlayerVersioning::V0
    }

    fn update_to_at_least_version(&self, version: Self::Version) -> Self {
        self.clone()
    }

    fn initial_state() -> Self {
        PlayerState {
            house: vec![vec![1]],
            version: PlayerVersioning::V0,
        }
    }

    fn apply_message(&self, message: PlayerMessage) -> Self {
        return match message {
            PlayerMessage::VersionMarker(version) => PlayerState {
                house: self.house.clone(),
                version,
            },
            PlayerMessage::Death => PlayerState {
                house: vec![],
                version: self.version,
            },
            PlayerMessage::Birth(_) => self.clone(),
            PlayerMessage::Build(vec) => {
                let mut house = self.house.clone();
                house.push(vec);
                PlayerState {
                    house,
                    version: self.version,
                }
            }
        };
    }
}

enum PlayerMessage {
    VersionMarker(PlayerVersioning),
    Birth(u32),
    Build(Vec<i32>),
    Death,
}

struct Line {
    key: String,
    // message: Message,
}

enum CompiledLine<T> {
    LARGE(String, String),
    SMALL(String, T),
}*/
