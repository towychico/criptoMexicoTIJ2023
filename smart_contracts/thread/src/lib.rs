#![no_std]

use thread_io::{InitThread, Thread, ThreadAction, ThreadState};
use gstd::{ msg, prelude::*};

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
    // TODO transfer 1 token to admin ?
}

#[no_mangle]
extern "C" fn state() {
    let state: &mut Thread =
        state_mut();
    msg::reply(state, 0).expect("failed to encode or reply from `state()`");
}

