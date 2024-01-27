use uuid::Uuid;
use crate::domain::id::IdRepository;

pub struct UuidRepository;

impl IdRepository for UuidRepository {

    fn get_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }
}