use std::{fs, path::Path};

use crate::prelude::*;

use super::{FileRepository, FileType};

pub struct FsFileRepository {
    base_path: String,
}

impl FsFileRepository {
    pub fn new(settings: &Arc<WispConfig>) -> Self {
        FsFileRepository {
            base_path: settings.file.fs_storage_path.clone(),
        }
    }
}

impl FileRepository for FsFileRepository {
    fn save_file(&mut self, file_type: FileType, name: String, bytes: &[u8]) -> Result<()> {
        let base_path = &self.base_path.clone();
        let base_path = Path::new(base_path).join(file_type.to_string());
        let path = base_path.join(name);

        fs::create_dir_all(base_path)?;
        fs::write(path, bytes)?;

        Ok(())
    }

    fn get_file(&mut self, file_type: FileType, name: String) -> Result<Option<Vec<u8>>> {
        let base_path = &self.base_path.clone();
        let path = Path::new(base_path).join(file_type.to_string()).join(name);

        if !path.exists() {
            return Ok(None);
        }

        let bytes = fs::read(path)?;
        Ok(Some(bytes))
    }
}
