use post_io::{InitPost, PostAction};
use gstd::ActorId;
use gtest::{Program, System};

const USER: u128 = 100;
const LIKES: u128 = 0;
const ID: u128 = 1;
const CONTENT: &str = "ALOOO";

#[test]
fn handle() {
    let sys = System::new();
    sys.init_logger();
    let post = Program::current(&sys);
    let res = post.send(2,
    InitPost {
        id: ID,
        poster: USER,
        likes: LIKES,
        content: CONTENT
    }
    );
    assert!(!res.main_failed());
    post.send(2, PostAction::Like);
}
