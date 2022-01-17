use super::{Ack, Record, SingleRecord};
use std::collections::{HashMap, HashSet};
/// Stores sequence numbers and their relevant data sets.
#[derive(Clone, Debug)]
pub struct AckQueue {
    current: u32,
    queue: HashMap<u32, Vec<u8>>,
}

impl AckQueue {
    pub fn new() -> Self {
        Self {
            current: 0,
            queue: HashMap::new(),
        }
    }

    pub fn make_ack(&mut self) -> Ack {
        let mut records: Vec<Record> = Vec::new();

        for (seq, _) in self.queue.clone().iter() {
            self.drop_seq(*seq);
            records.push(Record::Single(SingleRecord { sequence: *seq }));
        }

        let mut ack = Ack::new(records.len() as u16, true);
        ack.records = records;

        ack
    }

    pub fn increment_seq(&mut self, by: Option<u32>) {
        self.current += by.unwrap_or(1);
    }

    pub fn push_seq(&mut self, idx: u32, val: Vec<u8>) {
        self.queue.insert(idx, val);
    }

    pub fn drop_seq(&mut self, idx: u32) -> bool {
        if self.queue.contains_key(&idx) {
            self.queue.remove_entry(&idx);
            true
        } else {
            false
        }
    }

    pub fn get_seq(&self, idx: u32) -> Option<&Vec<u8>> {
        if self.queue.contains_key(&idx) {
            self.queue.get(&idx)
        } else {
            None
        }
    }

    pub fn has_seq(&self, idx: u32) -> bool {
        self.queue.contains_key(&idx)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }
}

#[derive(Clone, Debug)]
pub struct NAckQueue {
    current: u32,
    queue: HashSet<u32>,
}

impl NAckQueue {
    pub fn new() -> Self {
        Self {
            current: 0,
            queue: HashSet::new(),
        }
    }

    pub fn push_seq(&mut self, idx: u32) {
        self.queue.insert(idx);
    }

    pub fn drop_seq(&mut self, idx: u32) -> bool {
        if self.queue.contains(&idx) {
            self.queue.remove(&idx);
            true
        } else {
            false
        }
    }

    pub fn get_seq(&self, idx: u32) -> Option<u32> {
        if self.queue.contains(&idx) {
            Some(idx)
        } else {
            None
        }
    }

    pub fn has_seq(&self, idx: u32) -> bool {
        self.queue.contains(&idx)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }

    pub fn make_nack(&mut self) -> Ack {
        let mut records: Vec<Record> = Vec::new();

        for seq in self.queue.clone().iter() {
            self.drop_seq(*seq);
            records.push(Record::Single(SingleRecord { sequence: *seq }));
        }

        let mut ack = Ack::new(records.len() as u16, false);
        ack.records = records;

        ack
    }

    pub fn increment_seq(&mut self, by: Option<u32>) {
        self.current += by.unwrap_or(1);
    }
}
