use uuid::Uuid;

pub trait IdRepository {
    fn get_uuid(&self) -> Uuid;
}

pub struct IdService {
    repository: Box<dyn IdRepository + Send + Sync>
}

impl IdService {
    pub fn new(repository: impl IdRepository + Send + Sync + 'static) -> Self {
        IdService { repository: Box::new(repository) }
    }

    pub async fn generate_uuid(&self) -> Uuid {
        self.repository.get_uuid()
    }
}

#[cfg(test)]
mod tests {
    use rocket::async_test;
    use crate::infrastructure::id_config::{UuidRepository};
    use super::*;

    #[async_test]
    async fn test_generate_uuid() {
        let id_service = IdService::new(UuidRepository);

        let generated_id_1 = id_service.generate_uuid().await;
        let generated_id_2 = id_service.generate_uuid().await;

        assert_ne!(generated_id_1, generated_id_2)
    }
}