use std::fs;
use std::path::PathBuf;

use crate::MainWindow;
use serde::{Deserialize, Serialize};
use slint::{CloseRequestResponse, ComponentHandle, PhysicalPosition, PhysicalSize};

const APP_CONFIG_DIR: &str = "Rustic Calculator";
const STATE_FILE: &str = "window-state.json";
const MIN_WIDTH: u32 = 360;
const MIN_HEIGHT: u32 = 560;
const MAX_WIDTH: u32 = 760;
const MAX_HEIGHT: u32 = 760;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct SavedWindowState {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

pub fn restore(ui: &MainWindow) {
    let Some(state) = load() else {
        return;
    };

    let scale_factor = ui.window().scale_factor();
    let size = clamp_size(state.width, state.height, scale_factor);
    ui.window().set_size(size);

    if is_usable_position(state.x, state.y) {
        ui.window()
            .set_position(PhysicalPosition::new(state.x, state.y));
    }
}

pub fn save_on_close(ui: &MainWindow) {
    let ui_handle = ui.as_weak();

    ui.window().on_close_requested(move || {
        if let Some(ui) = ui_handle.upgrade() {
            save(&ui);
        }

        CloseRequestResponse::HideWindow
    });
}

fn load() -> Option<SavedWindowState> {
    let path = state_path()?;
    let contents = fs::read_to_string(path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn save(ui: &MainWindow) {
    let Some(path) = state_path() else {
        return;
    };

    let size = ui.window().size();
    let position = ui.window().position();
    let state = SavedWindowState {
        width: size.width,
        height: size.height,
        x: position.x,
        y: position.y,
    };

    let Some(parent) = path.parent() else {
        return;
    };

    if fs::create_dir_all(parent).is_err() {
        return;
    }

    let Ok(contents) = serde_json::to_string_pretty(&state) else {
        return;
    };

    let _ = fs::write(path, contents);
}

fn clamp_size(width: u32, height: u32, scale_factor: f32) -> PhysicalSize {
    let minimum = logical_size_to_physical(MIN_WIDTH, MIN_HEIGHT, scale_factor);
    let maximum = logical_size_to_physical(MAX_WIDTH, MAX_HEIGHT, scale_factor);

    PhysicalSize::new(
        width.clamp(minimum.width, maximum.width),
        height.clamp(minimum.height, maximum.height),
    )
}

fn logical_size_to_physical(width: u32, height: u32, scale_factor: f32) -> PhysicalSize {
    PhysicalSize::new(
        (width as f32 * scale_factor).round() as u32,
        (height as f32 * scale_factor).round() as u32,
    )
}

fn state_path() -> Option<PathBuf> {
    config_root().map(|root| root.join(APP_CONFIG_DIR).join(STATE_FILE))
}

fn config_root() -> Option<PathBuf> {
    std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from))
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .map(|home| home.join(".config"))
        })
}

#[cfg(target_os = "windows")]
fn is_usable_position(x: i32, y: i32) -> bool {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetSystemMetrics, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
        SM_YVIRTUALSCREEN,
    };

    let screen_x = unsafe { GetSystemMetrics(SM_XVIRTUALSCREEN) };
    let screen_y = unsafe { GetSystemMetrics(SM_YVIRTUALSCREEN) };
    let screen_width = unsafe { GetSystemMetrics(SM_CXVIRTUALSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYVIRTUALSCREEN) };

    let max_x = screen_x + screen_width;
    let max_y = screen_y + screen_height;

    x >= screen_x - 40 && x <= max_x - 80 && y >= screen_y && y <= max_y - 80
}

#[cfg(not(target_os = "windows"))]
fn is_usable_position(x: i32, y: i32) -> bool {
    x > -10_000 && y > -10_000
}
