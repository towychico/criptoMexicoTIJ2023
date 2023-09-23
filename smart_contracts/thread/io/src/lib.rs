#![no_std]
use gstd::{ActorId, prelude::*, msg};
use gmeta::{In, InOut, Metadata};

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub struct Reply {
    pub post_id: u128,
    pub post_owner: ActorId,
    pub content: String,
    pub number_of_likes: u128,
    pub number_of_reports: u128,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct InitThread {
    pub id: u128,
    pub owner: ActorId,
    pub thread_type: ThreadType,
    pub content: String
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Thread {
    pub id: u128,
    pub owner: ActorId,
    pub thread_type: ThreadType,
    pub content: String,
    pub replies: Vec<Reply>,
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
    AddReply(Reply),
    LikeReply(u128)
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Clone, Debug)]
pub enum ThreadEvent {
    ThreadEnded,
    ReplyAdded,
    ReplyLiked
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitThread>;
    type Handle = InOut<ThreadAction,ThreadEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Thread;
}

pub struct TokensToDistribute {
    pub tokens_for_winner: f64,
}

impl TokensToDistribute {
    pub fn new() -> Self {
        Self {
            tokens_for_winner: 0.0,
        }
    }
    pub fn calculate_tokens_to_distribute(&mut self, total_tokens: u128) {
        self.tokens_for_winner = (total_tokens as f64) * 0.4;
    }
    pub async fn transfer_tokens(&mut self, winner_id: ActorId) {
        let FT_PROGRAM_ID: &str = "0x670ac04961801362465fefc9a65745bfdc6119f597d378a1e19fc5696abba058";
        let FT_PROGRAM_ACTORID: ActorId = ActorId::from_bs58(FT_PROGRAM_ID.to_string()).expect("Actor ID not retrieved");
        let ADMIN_ID_ADDRESS: &str = "0xdc2c68689fa7c7c808105c0a12b26b976dd9ea031e4389da864938b00e0b7f16";
        let ADMIN_ID_ACTORID: ActorId = ActorId::from_bs58(ADMIN_ID_ADDRESS.to_string()).expect("Actor ID not retrieved");
        // transfer 40% of distributed tokens to winner
        msg::send(FT_PROGRAM_ACTORID, FTAction::Transfer {
            from: ADMIN_ID_ACTORID,
            to: winner_id,
            amount: self.tokens_for_winner as u128
        }, 0).expect("Transfer not completed");
    }
}

impl Thread {
    pub fn find_reply_by_id(&mut self, target_id: u128) -> Option<&mut Reply> {
        for reply in self.replies.iter_mut() {
            if reply.post_id == target_id {
                return Some(reply);
            }
        }
        None
    }
    pub fn get_winner(&mut self) -> Option<&ActorId> {
        let mut max_likes = 0;
        let mut actor_id_with_most_likes = None;

        for reply in &self.replies {
            if reply.number_of_likes > max_likes {
                max_likes = reply.number_of_likes;
                actor_id_with_most_likes = Some(&reply.post_owner);
            }
        }
        actor_id_with_most_likes
    }
    pub fn end_thread(&mut self) {
        self.state = ThreadState::Expired;
        // distribute tokens: 40% to absolute winner, 40% to path, and 20% to the rest of top n posts.
        let mut tokens_to_distribute: TokensToDistribute = TokensToDistribute::new();

        let distributed_tokens_clone = self.distributed_tokens.clone();
        let winner: &ActorId = self.get_winner().expect("Couldn't get ActorId of Winner");

        tokens_to_distribute.calculate_tokens_to_distribute(distributed_tokens_clone);
        tokens_to_distribute.transfer_tokens(*winner);
    }
    pub fn add_reply(&mut self, reply: Reply) {
        let FT_PROGRAM_ID: &str = "0x670ac04961801362465fefc9a65745bfdc6119f597d378a1e19fc5696abba058";
        let FT_PROGRAM_ACTORID: ActorId = ActorId::from_bs58(FT_PROGRAM_ID.to_string()).expect("Actor ID not retrieved");
        let ADMIN_ID_ADDRESS: &str = "0xdc2c68689fa7c7c808105c0a12b26b976dd9ea031e4389da864938b00e0b7f16";
        let ADMIN_ID_ACTORID: ActorId = ActorId::from_bs58(ADMIN_ID_ADDRESS.to_string()).expect("Actor ID not retrieved");
        let cloned_reply = reply.clone();
        self.replies.push(cloned_reply);
        // transfer x token to admin or escrow
        msg::send(FT_PROGRAM_ACTORID, FTAction::Transfer {
            from: reply.post_owner,
            to: ADMIN_ID_ACTORID,
            amount: 1
        }, 0).expect("Transfer not completed");
        self.distributed_tokens += 1;
    }
    pub fn like_reply(&mut self, post_id: u128) {
        let FT_PROGRAM_ID: &str = "0x670ac04961801362465fefc9a65745bfdc6119f597d378a1e19fc5696abba058";
        let FT_PROGRAM_ACTORID: ActorId = ActorId::from_bs58(FT_PROGRAM_ID.to_string()).expect("Actor ID not retrieved");
        let ADMIN_ID_ADDRESS: &str = "0xdc2c68689fa7c7c808105c0a12b26b976dd9ea031e4389da864938b00e0b7f16";
        let ADMIN_ID_ACTORID: ActorId = ActorId::from_bs58(ADMIN_ID_ADDRESS.to_string()).expect("Actor ID not retrieved");
        // search for reply with that id in replies
        let reply: &mut Reply = self.find_reply_by_id(post_id).expect("Reply not found");
        // increment their number of likes
        reply.number_of_likes += 1;
        // transfer x token to admin or escrow
        msg::send(FT_PROGRAM_ACTORID, FTAction::Transfer {
            from: reply.post_owner,
            to: ADMIN_ID_ACTORID,
            amount: 1
        }, 0).expect("Transfer not completed");
        self.distributed_tokens += 1;
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}