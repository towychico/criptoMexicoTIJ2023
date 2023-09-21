#![no_std]
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use thread_io::*;

#[metawasm]
pub mod metafns {
    pub type State = Thread;

    pub fn owner(state: State) -> ActorId {
        state.owner
    }

    pub fn get_likes(state: State) -> u128 {
        state.likes
    }

    pub fn get_id(state: State) -> String {
        state.id
    }
}