use crate::{
    app::StatusOr,
    file::MmapFile,
};
use std::{
    ffi::OsString,
    fs::{
        File,
        self
    },
    io::{
        BufReader,
        Read,
    },
    path::{
        Path,
        PathBuf
    },
};

lazy_static! {
  static ref RESOURCE_BASE: PathBuf = try_find_resource_base().unwrap();
}

pub fn resource_base() -> PathBuf {
    RESOURCE_BASE.to_path_buf()
}

pub fn reader(path: &PathBuf) -> StatusOr<BufReader<File>> {
    let file = File::open(path)
        .map_err(|e| format!("Error opening file {:?}: {}", path, e))?;
    Ok(BufReader::new(file))
}

pub fn slurp_file(path: &PathBuf) -> StatusOr<String> {
    let file = File::open(path)
        .map_err(|e| format!("Error opening file {:?}: {}", path, e))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
        .map_err(|e| format!("Error reading to string from slurp_file {:?}: {}", path, e))?;
    Ok(contents)
}

pub fn mmap(path: &PathBuf) -> StatusOr<MmapFile> {
    MmapFile::read(path)
}

pub fn resource_path(parent_folder: &'static str, resource_name: &'static str) -> PathBuf {
    let mut path_buf = resource_base();
    [parent_folder, resource_name].iter().for_each(|p| path_buf.push(p));
    path_buf
}

fn dir_contains_res(path: &Path) -> StatusOr<bool> {
    for entry in fs::read_dir(path)
        .map_err(|e| format!("Couldn't read dir {:?}: {}", path, e))? {
        let entry = entry
            .map_err(|e| format!("Couldn't read entry in {:?}: {}", path, e))?;
        if entry.file_name() == OsString::from("res") {
            return Ok(true)
        }
    }
    Ok(false)
}

fn try_find_resource_base() -> StatusOr<PathBuf> {
    let mut path_buf = PathBuf::from(".").canonicalize()
        .map_err(|e| format! ("Couldn't canonicalize CWD: {}", e))?;

    // TODO: Apply this call recursively instead of stopping.
    if dir_contains_res(path_buf.as_path())? {
        path_buf.push("res");
        Ok(path_buf)
    } else {
       Err(String::from("Could not find resource folder!"))
    }
}
