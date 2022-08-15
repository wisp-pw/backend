use std::sync::{Arc, RwLock};

use crate::prelude::*;
use crate::repositories::file::*;

pub struct FileSaveService {
    repo: Box<dyn FileRepository + Send + Sync>,
}

impl FileSaveService {
    pub fn new(repo: Box<dyn FileRepository + Send + Sync>) -> SharedFileSaveService {
        Arc::new(RwLock::new(FileSaveService { repo }))
    }

    pub fn save(&mut self, file_type: FileType, name: String, bytes: &[u8]) -> Result<()> {
        self.repo.save_file(file_type, name, bytes)
    }
}

pub type SharedFileSaveService = Arc<RwLock<FileSaveService>>;
