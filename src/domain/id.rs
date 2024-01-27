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
    use crate::infrastructure::id_config::{UuidRepository};
    use super::*;

    #[async_test]
    async fn test_generate_uuid() {
        let id_service = IdService::new(UuidRepository);

        let generated_id_1 = id_service.generate().await;
        let generated_id_2 = id_service.generate().await;

        assert_ne!(generated_id_1, generated_id_2)
    }
}