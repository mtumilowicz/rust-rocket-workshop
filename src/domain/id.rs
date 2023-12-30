pub trait IdRepository {
    fn get(&self) -> String;
}

pub struct IdService {
    repository: Box<dyn IdRepository + Send + Sync>
}

impl IdService {
    pub fn new(repository: impl IdRepository + Send + Sync + 'static) -> Self {
        IdService { repository: Box::new(repository) }
    }

    pub async fn generate(&self) -> String {
        self.repository.get()
    }
}

#[cfg(test)]
mod tests {
    use rocket::async_test;
    use crate::infrastructure::id_config::{DeterministicRepository, UuidRepository};
    use super::*;

    #[async_test]
    async fn test_generate_uuid() {
        let id_service = IdService::new(UuidRepository);

        let generated_id_1 = id_service.generate().await;
        let generated_id_2 = id_service.generate().await;

        assert_ne!(generated_id_1, generated_id_2)
    }

    #[async_test]
    async fn test_generate_deterministic() {
        let id_service = IdService::new(DeterministicRepository::new());

        let generated_id_1 = id_service.generate().await;
        let generated_id_2 = id_service.generate().await;

        assert_eq!(generated_id_1, "0");
        assert_eq!(generated_id_2, "1");
    }
}