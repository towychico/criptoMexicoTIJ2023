#![no_std]

use post_io::{InitPost, Post, PostAction};
use gstd::{msg, prelude::*};

static mut POST: Option<Post> = None;

#[no_mangle]
extern "C" fn handle() {
    let action: PostAction = msg::load()
        .expect("Unable to decode `PostAction`");
    let post: &mut Post = unsafe {
        POST
            .as_mut()
            .expect("The contract is not initialized")
    };
    match action {
        PostAction::Like => post.like()
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitPost = msg::load()
        .expect("Error in decoding `InitPost`");
    let post = Post {
        poster: init_config.poster,
        likes: init_config.likes,
        id: init_config.id,
        content: init_config.content
    };
    unsafe { POST = Some(post) };
}

#[no_mangle]
extern "C" fn state() {
    let post = unsafe {
        POST.get_or_insert(Default::default())
    };
    msg::reply(post,0)
        .expect("Failed to share state");
}
