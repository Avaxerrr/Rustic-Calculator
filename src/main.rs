mod app;
mod calculator;
mod converter_app;
mod features;
mod native_window;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    native_window::apply_when_ready("Rustic Calculator");
    app::run(ui)
}
