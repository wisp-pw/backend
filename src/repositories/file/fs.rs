use std::{fs, path::Path};

use crate::prelude::*;

use super::FileRepository;

pub struct FsFileRepository {
    base_path: String,
}

impl FsFileRepository {
    pub fn new(settings: &Arc<WispSettings>) -> Self {
        FsFileRepository {
            base_path: settings.fs_storage_path.clone(),
        }
    }
}

impl FileRepository for FsFileRepository {
    fn save_file(&mut self, name: String, bytes: &[u8]) -> Result<()> {
        let base_path = &self.base_path.clone();
        let path = Path::new(base_path).join(name);

        fs::write(path, bytes)?;

        Ok(())
    }

    fn get_file(&mut self, name: String) -> Option<&bytes::Bytes> {
        todo!()
    }
}
