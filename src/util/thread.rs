
enum ThreadState {
    Stopped,
    Waiting,
    Running,
    Blocked
}
pub struct Thread {
    state: ThreadState,
    priority: u8,
}