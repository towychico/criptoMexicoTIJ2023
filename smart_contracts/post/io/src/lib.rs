#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, debug};
use scale_info::TypeInfo;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitPost>;
    type Handle = InOut<PostAction, PostEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Post;
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitPost {
    pub id: u128,
    pub poster: u128,
    pub likes: u128,
    pub content: String
}

#[derive(Encode, Decode, TypeInfo)]
pub enum PostAction {
    Like
}

#[derive(Encode, Decode, TypeInfo)]
pub enum PostEvent {
    Liked
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Post {
    pub id: u128,
    pub poster: u128,
    pub likes: u128,
    pub content: String
}

impl Post {
    pub fn like(&mut self) {
        self.likes = self.likes + 1;
        debug!("HOW MANY LIKES? {:?}", self.likes);
    }
}