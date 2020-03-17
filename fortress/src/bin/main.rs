#![windows_subsystem = "windows"]
extern crate fortress;

mod mains;

fn main() -> std::result::Result<(), String> {
    mains::app::main()
}
