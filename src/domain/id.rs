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