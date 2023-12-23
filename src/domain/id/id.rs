trait IdRepository {
    fn get() -> String;
}

struct IdService {
    repository: Box<dyn IdRepository + Send + Sync>
}

impl IdService {
    pub fn new(repository: impl IdRepository + Send + Sync + 'static) -> Self {
        HelloRequestService { repository: Box::new(repository) }
    }

    pub fn generate(&self) -> String {
        self.repository.get();
    }
}