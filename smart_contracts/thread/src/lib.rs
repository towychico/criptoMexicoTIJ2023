#![no_std]

use thread_io::{FTAction, InitThread, Thread, ThreadAction, ThreadState};
use gstd::{ActorId, msg, prelude::*};

static mut THREAD:Option<Thread> = None;

fn state_mut() -> &'static mut Thread {
    let state = unsafe { THREAD.as_mut()};
    unsafe { state.unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn handle() {
    let action: ThreadAction = msg::load()
        .expect("Unable to decode `ThreadAction`");
    let thread: &mut Thread = unsafe {
        THREAD
            .as_mut()
            .expect("The contract is not initialized")
    };
    match action {
        ThreadAction::EndThread => thread.end_thread(),
        ThreadAction::AddReply(post) => thread.add_reply(post),
        ThreadAction::LikeReply(post_id) => thread.like_reply(post_id)
    }
}

#[no_mangle]
extern "C" fn init() {
    let ADMIN_ID_ADDRESS: &str = "0xdc2c68689fa7c7c808105c0a12b26b976dd9ea031e4389da864938b00e0b7f16";
    let ADMIN_ID_ACTORID: ActorId = ActorId::from_bs58(ADMIN_ID_ADDRESS.to_string()).expect("Actor ID not retrieved");
    let FT_PROGRAM_ID: &str = "0x670ac04961801362465fefc9a65745bfdc6119f597d378a1e19fc5696abba058";
    let FT_PROGRAM_ACTORID: ActorId = ActorId::from_bs58(FT_PROGRAM_ID.to_string()).expect("Actor ID not retrieved");
    let init_config: InitThread = msg::load()
        .expect("Error in decoding `InitThread`");
    let thread = Thread {
        id: init_config.id,
        owner: init_config.owner,
        thread_type: init_config.thread_type,
        content: init_config.content,
        replies: Vec::new(),
        state: ThreadState::Active,
        distributed_tokens: 1 // consider token transferred for creating the post.
    };
    unsafe { THREAD = Some(thread) };
    // transfer 1 token to admin
    msg::send(FT_PROGRAM_ACTORID, FTAction::Transfer {
        from: msg::source(),
        to: ADMIN_ID_ACTORID,
        amount: 1
    }, 0).expect("Transfer not completed");
}

#[no_mangle]
extern "C" fn state() {
    let state: &mut Thread =
        state_mut();
    msg::reply(state, 0).expect("failed to encode or reply from `state()`");
}

