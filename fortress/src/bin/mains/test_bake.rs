use fortress_bake::{
    app::StatusOr,
    bake,
};
use std::path::PathBuf;

pub fn main() -> StatusOr<()> {
    let mut root = PathBuf::new();
    root.push(".");
    bake::run(root)
}
