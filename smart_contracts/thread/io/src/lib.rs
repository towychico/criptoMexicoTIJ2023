#![no_std]
use gstd::{prelude::*};
use gmeta::{InOut,Metadata};

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct Post {
    pub post_id: String,
    pub user_name: String,
    pub number_of_likes: u64,
    pub number_of_reports: u64,
    pub number_of_saves: u64,
    pub post_number: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitThread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub thread_type: ThreadType,
    pub content: String
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Thread {
    pub id: u128,
    pub owner: u128, // Change later to ActorID
    pub thread_type: ThreadType,
    pub content: String,
    pub replies: Vec<Post>, // Cambiar a Graph<Post>
    pub state: ThreadState,
    pub distributed_tokens: u128
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ThreadType {
    Challenge,
    Question
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadState {
    Active,
    Expired
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadAction {
    EndThread,
    AddReply(Post),
    LikeReply(String)

}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadEvent {
    ThreadEnded,
    ReplyAdded,
    ReplyLiked
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

pub struct TokensToDistribute {
    pub tokens_for_winner: f64,
    pub tokens_for_each_winner_route: f64,
    pub tokens_for_each_top: f64
}

impl TokensToDistribute {
    pub fn new() -> Self {
        Self {
            tokens_for_winner: 0.0,
            tokens_for_each_winner_route: 0.0,
            tokens_for_each_top: 0.0,
        }
    }
    pub fn calculate_tokens_to_distribute(&mut self, total_tokens: u128, n_winners_route: u128, n_top: u128) {
        self.tokens_for_winner = (total_tokens as f64) * 0.4;
        self.tokens_for_each_winner_route = ((total_tokens as f64) * 0.4) / (n_winners_route as f64);
        self.tokens_for_each_top = ((total_tokens as f64) * 0.2) / (n_top as f64);
    }
    pub fn transfer_tokens(&mut self) {
        // TODO transfer 40% of distributed tokens to winner ?
        // msg.send(program_id, transfer,
    }
}

impl Thread {
    pub fn end_thread(&mut self) {
        self.state = ThreadState::Expired;
        // distribute tokens: 40% to absolute winner, 40% to path, and 20% to the rest of top n posts.

        let tokens_to_distribute: TokensToDistribute = TokensToDistribute::new();

        // let winner: ActorId = get_winner();
        // let winner_path: Vec<ActorId> = get_winner_path(); ?
        // let top_posters: Vec<ActorId> = get_top_posters(); ?

        // tokens_to_distribute.calculate_tokens_to_distribute(self.distributed_tokens, winner_path.len(), top_posters.len())
    }
    pub fn add_reply(&mut self, post: Post) {
        self.replies.push(post);
        // TODO transfer x token to admin or escrow ?

        self.distributed_tokens += 1;
    }
    pub fn like_reply(&mut self, post_id: String) {
        // search for reply with that id in replies
        // let reply: &post = get_reply_by_id(post_id)
        // increment their number of likes
        // reply.number_of_likes += 1;
        // TODO transfer x token to admin or escrow ?

        self.distributed_tokens += 1;
    }
}