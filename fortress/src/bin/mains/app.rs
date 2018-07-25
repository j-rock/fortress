use fortress::app::AppRunner;

pub fn main() {
    let mut app_runner = AppRunner::new().unwrap();
    app_runner.run().unwrap();
}
