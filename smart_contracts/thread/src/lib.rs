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
        ThreadAction::EndThread => thread.end_thread()
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitThread = msg::load()
        .expect("Error in decoding `InitThread`");
    let thread = Thread {
        id: init_config.id,
        owner: init_config.owner,
        post_type: init_config.post_type,
        content: init_config.content,
        replies: init_config.replies,
        state: ThreadState::Active
    };
    unsafe { THREAD = Some(thread) };
}

#[no_mangle]
extern "C" fn state() {
    let state: &mut Thread =
        state_mut();

    msg::reply(state, 0).expect("failed to encode or reply from `state()`");
}

