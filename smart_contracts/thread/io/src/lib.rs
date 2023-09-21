#![no_std]
use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, debug};
use scale_info::TypeInfo;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitThread>;
    type Handle = InOut<ThreadAction, ThreadEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Thread;
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitThread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub post_type: String,
    pub content: String,
    pub replies: String
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ThreadAction {
    EndThread
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ThreadEvent {
    ThreadEnded
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ThreadState {
    Active,
    Expired
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Thread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub post_type: String,
    pub content: String,
    pub replies: String,
    pub state: ThreadState
}

impl Thread {
    pub fn end_thread(&mut self) {
        self.state = ThreadState::Expired;
        // distribute tokens: 40% to absolute winner, 40% to path, and 20% to the rest of top n posts.
    }
}