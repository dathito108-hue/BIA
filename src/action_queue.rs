use std::collections::VecDeque;

use crate::capability::DeviceAction;

#[derive(Clone, Debug)]
pub struct ActionQueue {
    capacity: usize,
    items: VecDeque<DeviceAction>,
}

impl ActionQueue {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, action: DeviceAction) {
        if self.items.len() >= self.capacity {
            self.items.pop_front();
        }
        self.items.push_back(action);
    }

    pub fn front(&self) -> Option<&DeviceAction> {
        self.items.front()
    }

    pub fn pop_front(&mut self) -> Option<DeviceAction> {
        self.items.pop_front()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn items(&self) -> impl Iterator<Item = &DeviceAction> {
        self.items.iter()
    }
}
