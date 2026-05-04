use std::fs;
use std::path::PathBuf;

use crate::MainWindow;
use serde::{Deserialize, Serialize};
use slint::{
    CloseRequestResponse, ComponentHandle, LogicalSize, PhysicalPosition, PhysicalSize, Timer,
    TimerMode,
};

const APP_DIR_NAME: &str = "Rustic Calculator";
const STATE_FILE_NAME: &str = "window-state.json";

const DEFAULT_WIDTH: f32 = 400.0;
const DEFAULT_HEIGHT: f32 = 620.0;
const MIN_WIDTH: f32 = 360.0;
const MIN_HEIGHT: f32 = 560.0;
const MAX_WIDTH: f32 = 900.0;
const MAX_HEIGHT: f32 = 1000.0;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct WindowState {
    x: i32,
    y: i32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct ScreenBounds {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
}

pub fn restore_or_center(ui: &MainWindow) {
    let window = ui.window();
    let scale_factor = window.scale_factor();

    if let Some(state) = load_state() {
        let size = logical_size(state.width, state.height);
        let physical_size = PhysicalSize::from_logical(size, scale_factor);
        window.set_size(size);
        window.set_position(clamp_position(
            PhysicalPosition::new(state.x, state.y),
            physical_size,
        ));
        return;
    }

    let size = LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    window.set_size(size);
    window.set_position(center_position(PhysicalSize::from_logical(
        size,
        scale_factor,
    )));
}

pub fn autosave_timer(ui: &MainWindow) -> Timer {
    let timer = Timer::default();
    let ui_handle = ui.as_weak();

    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(750),
        move || {
            if let Some(ui) = ui_handle.upgrade() {
                save(&ui);
            }
        },
    );

    timer
}
pub fn save_on_close(ui: &MainWindow) {
    let ui_handle = ui.as_weak();

    ui.window().on_close_requested(move || {
        if let Some(ui) = ui_handle.upgrade() {
            save(&ui);
        }

        let _ = slint::quit_event_loop();
        CloseRequestResponse::HideWindow
    });
}
pub fn save(ui: &MainWindow) {
    let window = ui.window();

    if window.is_fullscreen() || window.is_maximized() || window.is_minimized() {
        return;
    }

    let position = window.position();
    let size = window.size().to_logical(window.scale_factor());
    let state = WindowState {
        x: position.x,
        y: position.y,
        width: size.width.clamp(MIN_WIDTH, MAX_WIDTH),
        height: size.height.clamp(MIN_HEIGHT, MAX_HEIGHT),
    };

    let Some(path) = state_file_path() else {
        return;
    };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    if let Ok(contents) = serde_json::to_string_pretty(&state) {
        let _ = fs::write(path, contents);
    }
}

fn load_state() -> Option<WindowState> {
    let path = state_file_path()?;
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn state_file_path() -> Option<PathBuf> {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())?;

    Some(base.join(APP_DIR_NAME).join(STATE_FILE_NAME))
}

fn logical_size(width: f32, height: f32) -> LogicalSize {
    LogicalSize::new(
        width.clamp(MIN_WIDTH, MAX_WIDTH),
        height.clamp(MIN_HEIGHT, MAX_HEIGHT),
    )
}

fn center_position(size: PhysicalSize) -> PhysicalPosition {
    let bounds = screen_bounds();
    let x = bounds.left + (bounds.width - size.width as i32).max(0) / 2;
    let y = bounds.top + (bounds.height - size.height as i32).max(0) / 2;

    PhysicalPosition::new(x, y)
}

fn clamp_position(position: PhysicalPosition, size: PhysicalSize) -> PhysicalPosition {
    let bounds = screen_bounds();
    let max_x = bounds.left + (bounds.width - size.width as i32).max(0);
    let max_y = bounds.top + (bounds.height - size.height as i32).max(0);

    PhysicalPosition::new(
        position.x.clamp(bounds.left, max_x),
        position.y.clamp(bounds.top, max_y),
    )
}

#[cfg(target_os = "windows")]
fn screen_bounds() -> ScreenBounds {
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    ScreenBounds {
        left: 0,
        top: 0,
        width: unsafe { GetSystemMetrics(SM_CXSCREEN) },
        height: unsafe { GetSystemMetrics(SM_CYSCREEN) },
    }
}

#[cfg(not(target_os = "windows"))]
fn screen_bounds() -> ScreenBounds {
    ScreenBounds {
        left: 0,
        top: 0,
        width: 1920,
        height: 1080,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_logical_size_to_supported_range() {
        let size = logical_size(100.0, 1000.0);

        assert_eq!(size.width, MIN_WIDTH);
        assert_eq!(size.height, MAX_HEIGHT);
    }
}
