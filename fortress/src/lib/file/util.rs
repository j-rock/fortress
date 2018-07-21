use std::{
    convert::AsRef,
    fs::File,
    io::{
        BufReader,
        BufWriter,
        Lines,
        prelude::*,
    },
    path::Path,
    self,
};

pub fn slurp_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn lines<P: AsRef<Path>>(path: P) -> std::io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines())
}

pub fn buffered_writer_for<P: AsRef<Path>>(path: P) -> std::io::Result<BufWriter<File>> {
    let file = File::create(path)?;
    Ok(BufWriter::new(file))
}

pub fn resource_path(parent_folder: &'static str, resource_name: &'static str) -> String {
    format!("D:\\Programming\\IntelliJ\\Fortress\\fortress\\src\\res\\{}\\{}", parent_folder, resource_name)
}
