
enum ThreadState {
    Stopped,
    Waiting,
    Running,
    Blocked
}

pub struct ThreadManager {
    
}

pub struct Thread {
    state: ThreadState,
    priority: u8,
}