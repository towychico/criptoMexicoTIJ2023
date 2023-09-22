#![no_std]
use gstd::{prelude::*, ActorId, HashMap as GHashMap, msg};
use gmeta::{InOut,Metadata};

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct Post {
    pub user_name: String,
    pub number_of_likes: u64,
    pub number_of_reports: u64,
    pub number_of_saves: u64,
    pub post_id: String,
    pub post_number: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitThread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub thread_type: String,
    pub content: String
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Thread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub thread_type: String,
    pub content: String,
    pub replies: Vec<Post>, // Cambiar a Graph<Post>
    pub state: ThreadState
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadState {
    Active,
    Expired
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadAction {
    // Add Actions
    EndThread,
    AddReply
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadEvent {
    ThreadEnded
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = ();
    type Handle = InOut<ThreadAction,ThreadEvent>;
    type Reply= ();
    type Others = ();
    type Signal = ();
    type State = Thread;
}

impl Thread {
    pub fn end_thread(&mut self) {
        self.state = ThreadState::Expired;
        // distribute tokens: 40% to absolute winner, 40% to path, and 20% to the rest of top n posts.
        // send message to token contract to transfer from admin-escrow to each

        // let winner: ActorId = get_winner();
        // let winner_path: Vec<ActorId> = get_winner_path(); ?
        // let top_posters: Vec<ActorId> = get_top_posters(); ?
    }
    pub fn add_reply(&mut self) {
        // create node
        let post1 = Post{user_name:"Lou".to_string(),number_of_likes:0,number_of_reports:0,number_of_saves:0,post_id:"A01".to_string(),post_number:2};
        self.replies.push(post1);
        // TODO transfer x token to admin or escrow ?
    }
    pub fn like_reply(&mut self, post_id: String) {
        // search for reply with that id in replies
        // let reply: &reply = get_reply_by_id(post_id)
        // increment their number of likes
        // reply.number_of_likes += 1;
        // TODO transfer x token to admin or escrow ?
    }
}