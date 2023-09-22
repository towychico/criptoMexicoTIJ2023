use std::string::ToString;
use gstd::ActorId;
use gtest::{Program, System};
use thread_io::{InitThread, ThreadAction, ThreadType};

const ID: u128 = 100;
const OWNER: u128 = 100; // Change later to ActorID
const THREAD_TYPE: ThreadType = ThreadType::Challenge;
const CONTENT: &str = "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...";

#[test]
fn init() {
    let sys = System::new();
    sys.init_logger();
    let thread = Program::current(&sys);
    let res = thread.send(2,
    InitThread {
        id: ID,
        owner: OWNER,
        thread_type: THREAD_TYPE,
        content: CONTENT.parse().unwrap()
    }
    );
    assert!(!res.main_failed());
}

#[test]
fn handle() {
    let sys = System::new();
    sys.init_logger();
    let thread = Program::current(&sys);
    let res = thread.send(2, ThreadAction::EndThread);
    assert!(!res.main_failed());
}
