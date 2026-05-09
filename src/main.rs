#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod calculator;
mod clipboard;
mod converter_app;
mod features;
mod native_window;
mod number_input;
mod window_state;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    window_state::restore_or_center(&ui);
    window_state::save_on_close(&ui);
    native_window::apply_when_ready("Rustic Calculator");
    app::run(ui)
}
