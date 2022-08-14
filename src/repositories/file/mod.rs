mod fs;
mod memory;

use crate::prelude::*;

pub use self::fs::FsFileRepository;
pub use self::memory::MemoryFileRepository;

pub trait FileRepository {
    fn save_file(&mut self, name: String, bytes: &[u8]) -> Result<()>;

    fn get_file(&mut self, name: String) -> Option<&bytes::Bytes>;
}
