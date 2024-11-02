enum ThreadState {
    Stopped,
    Waiting,
    Running,
}
pub struct Thread {
    state: ThreadState,
    priority: u8,
}
