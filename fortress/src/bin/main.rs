#![windows_subsystem = "windows"]
extern crate fortress;
extern crate fortress_bake;

use fortress::app::StatusOr;

mod mains;

fn main() -> StatusOr<()> {
    mains::app::main()
}
