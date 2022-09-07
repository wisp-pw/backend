use hashbrown::HashMap;

use crate::prelude::*;

use super::{FileRepository, FileType};

pub struct MemoryFileRepository {
    files: HashMap<String, Vec<u8>>,
}

impl MemoryFileRepository {
    pub fn new() -> Self {
        MemoryFileRepository {
            files: HashMap::new(),
        }
    }
}

impl FileRepository for MemoryFileRepository {
    fn save_file(&mut self, _file_type: FileType, name: String, bytes: &[u8]) -> Result<()> {
        self.files.insert(name, bytes.to_vec());

        Ok(())
    }

    fn get_file(&mut self, _file_type: FileType, name: String) -> Result<Option<Vec<u8>>> {
        let bytes = self.files.get(&name);
        match bytes {
            Some(vec) => Ok(Some(vec.to_vec())),
            None => Ok(None),
        }
    }
}
