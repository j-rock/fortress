extern crate fortress_bake;

use fortress_bake::{
    app::StatusOr,
    bake::{
        self,
        InputOutput,
    },
};
use std::path::PathBuf;

fn main() -> StatusOr<()> {
    let mut root = PathBuf::new();
    root.push("..");
    let input_output = InputOutput::new(root)?;
    bake::run(input_output)
}