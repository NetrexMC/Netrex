use super::queue::{AckQueue, NAckQueue};
// Processes ack packets for the server.

pub struct AckProcessor {
    ack_queue: AckQueue,
    nack_queue: NAckQueue,
}

impl AckProcessor {
    pub fn new() -> Self {
        Self {
            ack_queue: AckQueue::new(),
            nack_queue: NAckQueue::new(),
        }
    }
}
