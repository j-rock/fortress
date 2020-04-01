extern crate fortress_bake;

use fortress_bake::{
    app::StatusOr,
    bake::{
        self,
        InputOutput,
    },
    file,
};
use std::path::PathBuf;

fn copy_font(input_output: &InputOutput) -> StatusOr<()> {
    let font = "veger_regular.ttf";
    let font_in = input_output.input.fonts.join(font);
    let font_out = input_output.output.fonts.join(font);
    file::util::copy_file(font_in, font_out)
}

fn main() -> StatusOr<()> {
    let mut root = PathBuf::new();
    root.push("..");
    let input_output = InputOutput::new(root)?;

    copy_font(&input_output)?;

    bake::run(input_output)
}