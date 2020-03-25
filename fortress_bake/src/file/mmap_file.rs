use crate::app::StatusOr;
use memmap::{
    Mmap,
    MmapOptions,
};
use std::{
    fs::File,
    path::PathBuf,
};

pub struct MmapFile {
    mmap: Mmap,
    _file: File,
}

impl MmapFile {
    pub fn read(path: &PathBuf) -> StatusOr<MmapFile> {
        let file = File::open(path)
            .map_err(|e| format!("Error opening file {:?}: {}", path, e))?;

        let mmap = unsafe {
            MmapOptions::new()
                .map(&file)
                .map_err(|err| format!("Couldn't mmap file: {}", err))?
        };

        Ok(MmapFile {
            mmap,
            _file: file,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.mmap
    }
}