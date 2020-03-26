#![windows_subsystem = "windows"]
extern crate fortress;
extern crate fortress_bake;

use fortress::app::StatusOr;

mod mains;

fn main() -> StatusOr<()> {
    if cfg!(feature = "bake") {
        mains::test_bake::main()?;
    }
    mains::app::main()
}
