use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

use songbird::input::AuxMetadata;

#[derive(Default)]
pub struct Data {
    queue_metadata: Arc<RwLock<VecDeque<AuxMetadata>>>,
}

impl Data {
    pub fn get_queue(&self) -> VecDeque<AuxMetadata> {
        self.queue_metadata.read().unwrap().clone()
    }

    pub fn get_track(&self, index: u32) -> Option<AuxMetadata> {
        self.queue_metadata
            .read()
            .unwrap()
            .get(index as usize)
            .cloned()
    }

    pub fn enqueue_track(&self, meta: AuxMetadata) {
        self.queue_metadata.write().unwrap().push_front(meta);
    }

    pub fn pop_track(&self) {
        self.queue_metadata.write().unwrap().pop_back().unwrap();
    }

    pub fn pop_range(&self, index: u32) {
        let mut queue = self.queue_metadata.write().unwrap();

        for _ in 0..index {
            queue.pop_back();
        }
    }

    pub fn clean(&self) {
        self.queue_metadata.write().unwrap().clear();
    }
}
