use uuid::Uuid;
use crate::domain::id::IdRepository;

pub struct UuidRepository;

impl IdRepository for UuidRepository {

    fn get(&self) -> String {
        Uuid::new_v4().to_string()
    }
}