use bytes::Bytes;
use hashbrown::HashMap;

use crate::prelude::*;

use super::FileRepository;

pub struct MemoryFileRepository {
    files: HashMap<String, Bytes>,
}

impl MemoryFileRepository {
    pub fn new() -> Self {
        MemoryFileRepository {
            files: HashMap::new(),
        }
    }
}

impl FileRepository for MemoryFileRepository {
    fn save_file(&mut self, name: String, bytes: &[u8]) -> Result<()> {
        self.files.insert(name, Bytes::copy_from_slice(bytes));

        Ok(())
    }

    fn get_file(&mut self, name: String) -> Option<&Bytes> {
        self.files.get(&name)
    }
}
