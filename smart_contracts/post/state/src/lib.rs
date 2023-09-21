#![no_std]
use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use post_io::*;

#[metawasm]
pub mod metafns {
    pub type State = Post;

    pub fn get_poster(state: State) -> ActorId {
        state.poster
    }

    pub fn get_likes(state: State) -> u128 {
        state.likes
    }

    pub fn get_id(state: State) -> String {
        state.id
    }
}