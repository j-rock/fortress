use fortress::app::{
    AppRunner,
    StatusOr,
};

pub fn main() -> StatusOr<()> {
    let mut app_runner = AppRunner::new()?;
    let _res = app_runner.run()?;
    std::process::exit(0);
}
