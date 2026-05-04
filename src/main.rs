mod app;
mod calculator;
mod converter_app;
mod features;
mod native_window;
mod window_state;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    window_state::restore(&ui);
    window_state::save_on_close(&ui);
    native_window::apply_when_ready("Rustic Calculator");
    app::run(ui)
}
