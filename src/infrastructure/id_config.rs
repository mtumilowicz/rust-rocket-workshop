use std::sync::atomic::{AtomicI32, Ordering};
use uuid::Uuid;
use crate::domain::id::{IdRepository, IdService};

pub struct UuidRepository;

impl IdRepository for UuidRepository {

    fn get(&self) -> String {
        Uuid::new_v4().to_string()
    }
}

pub struct DeterministicRepository {
    counter: AtomicI32,
}

impl DeterministicRepository {
    pub fn new() -> DeterministicRepository {
        DeterministicRepository {
            counter: AtomicI32::new(0)
        }
    }
}
impl IdRepository for DeterministicRepository {
    fn get(&self) -> String {
        self.counter.fetch_add(1, Ordering::Relaxed).to_string()
    }
}