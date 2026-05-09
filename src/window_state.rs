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
const HISTORY_MIN_WIDTH: f32 = 260.0;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
struct WindowState {
    x: i32,
    y: i32,
    width: f32,
    height: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    history_open: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    calculator_panel_width: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
struct RestoredLayout {
    size: LogicalSize,
    history_open: bool,
    calculator_panel_width: f32,
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
        let layout = restored_layout(state);
        let physical_size = PhysicalSize::from_logical(layout.size, scale_factor);
        ui.set_calculator_panel_width(layout.calculator_panel_width);
        ui.set_history_open(layout.history_open);
        ui.set_history_visible(layout.history_open);
        window.set_size(layout.size);
        window.set_position(clamp_position(
            PhysicalPosition::new(state.x, state.y),
            physical_size,
        ));
        return;
    }

    let size = LogicalSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    ui.set_calculator_panel_width(DEFAULT_WIDTH);
    ui.set_history_open(false);
    ui.set_history_visible(false);
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
    let width = size.width.clamp(MIN_WIDTH, MAX_WIDTH);
    let height = size.height.clamp(MIN_HEIGHT, MAX_HEIGHT);
    let history_open = ui.get_history_open();
    let calculator_panel_width = calculator_panel_width_for_save(ui, width, history_open);
    let state = WindowState {
        x: position.x,
        y: position.y,
        width,
        height,
        history_open: Some(history_open),
        calculator_panel_width: Some(calculator_panel_width),
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

fn restored_layout(state: WindowState) -> RestoredLayout {
    let mut size = logical_size(state.width, state.height);
    let history_open = state.history_open.unwrap_or(false);
    let calculator_panel_width = if history_open {
        let requested_width = state.calculator_panel_width.unwrap_or(DEFAULT_WIDTH);
        let panel_width = clamp_calculator_panel_width(requested_width, size.width, true);
        let minimum_open_width = (panel_width + HISTORY_MIN_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);

        if size.width < minimum_open_width {
            size.width = minimum_open_width;
        }

        panel_width
    } else {
        size.width
    };

    RestoredLayout {
        size,
        history_open,
        calculator_panel_width,
    }
}

fn calculator_panel_width_for_save(ui: &MainWindow, window_width: f32, history_open: bool) -> f32 {
    let width = if history_open {
        ui.get_calculator_panel_width()
    } else {
        window_width
    };

    clamp_calculator_panel_width(width, window_width, history_open)
}

fn clamp_calculator_panel_width(width: f32, window_width: f32, history_open: bool) -> f32 {
    if history_open {
        let max_panel_width = (window_width - HISTORY_MIN_WIDTH).clamp(MIN_WIDTH, MAX_WIDTH);
        width.clamp(MIN_WIDTH, max_panel_width)
    } else {
        width.clamp(MIN_WIDTH, MAX_WIDTH)
    }
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

    #[test]
    fn restores_legacy_state_as_closed_calculator() {
        let layout = restored_layout(WindowState {
            x: 0,
            y: 0,
            width: 720.0,
            height: 700.0,
            history_open: None,
            calculator_panel_width: None,
        });

        assert!(!layout.history_open);
        assert_eq!(layout.size.width, 720.0);
        assert_eq!(layout.calculator_panel_width, 720.0);
    }

    #[test]
    fn restores_open_history_with_calculator_width() {
        let layout = restored_layout(WindowState {
            x: 0,
            y: 0,
            width: 660.0,
            height: 700.0,
            history_open: Some(true),
            calculator_panel_width: Some(400.0),
        });

        assert!(layout.history_open);
        assert_eq!(layout.size.width, 660.0);
        assert_eq!(layout.calculator_panel_width, 400.0);
    }

    #[test]
    fn expands_too_small_open_history_state() {
        let layout = restored_layout(WindowState {
            x: 0,
            y: 0,
            width: 420.0,
            height: 700.0,
            history_open: Some(true),
            calculator_panel_width: Some(400.0),
        });

        assert!(layout.history_open);
        assert_eq!(layout.size.width, 620.0);
        assert_eq!(layout.calculator_panel_width, MIN_WIDTH);
    }
}
