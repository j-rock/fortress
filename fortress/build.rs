extern crate fortress_bake;

use fortress_bake::{
    app::StatusOr,
    bake,
};
use std::path::PathBuf;

fn main() -> StatusOr<()> {
    let mut root = PathBuf::new();
    root.push("..");
    bake::run(root)
}