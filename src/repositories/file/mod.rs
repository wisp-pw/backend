mod fs;
mod memory;

use crate::prelude::*;

pub use self::fs::FsFileRepository;
pub use self::memory::MemoryFileRepository;

pub enum FileType {
    Image,
}

impl ToString for FileType {
    fn to_string(&self) -> String {
        match self {
            FileType::Image => String::from("images"),
        }
    }
}

pub trait FileRepository {
    fn save_file(&mut self, file_type: FileType, name: String, bytes: &[u8]) -> Result<()>;

    fn get_file(&mut self, file_type: FileType, name: String) -> Result<Option<Vec<u8>>>;
}
