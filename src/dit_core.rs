
//! This module is for the core of the `dit` game, which should not change even
//! as the interface changes or has different interfaces.

mod action;
mod error;
mod game;
mod message;
mod state;
mod update;
mod validate;
mod work;

pub use action::spells::Spell;
pub use action::Action;
pub use error::{dit_result, io_error, Error};
pub use game::with_game_state;
pub use message::Message;
pub use state::State;
pub use update::{ToActionUpdate, Update};
pub use validate::validate;
