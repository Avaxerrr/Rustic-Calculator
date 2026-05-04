#[cfg(target_os = "windows")]
mod windows_title_bar {
    use std::thread;
    use std::time::Duration;

    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Dwm::{
        DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR, DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute,
    };
    use windows_sys::Win32::System::Threading::GetCurrentProcessId;
    use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};

    const APP_BACKGROUND: u32 = rgb_to_colorref(0x20, 0x20, 0x20);
    const APP_FOREGROUND: u32 = rgb_to_colorref(0xf2, 0xf2, 0xf2);

    pub fn apply_when_ready(title: &'static str) {
        thread::spawn(move || {
            for _ in 0..40 {
                if let Some(hwnd) = find_current_process_window(title) {
                    apply_to_hwnd(hwnd);
                    return;
                }

                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    fn find_current_process_window(title: &str) -> Option<HWND> {
        let title = wide_null(title);
        let hwnd = unsafe { FindWindowW(std::ptr::null(), title.as_ptr()) };

        if hwnd.is_null() {
            return None;
        }

        let mut window_process_id = 0;
        unsafe {
            GetWindowThreadProcessId(hwnd, &mut window_process_id);
        }

        (window_process_id == unsafe { GetCurrentProcessId() }).then_some(hwnd)
    }

    fn apply_to_hwnd(hwnd: HWND) {
        set_bool_attribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, true);
        set_color_attribute(hwnd, DWMWA_CAPTION_COLOR, APP_BACKGROUND);
        set_color_attribute(hwnd, DWMWA_TEXT_COLOR, APP_FOREGROUND);
    }

    fn wide_null(text: &str) -> Vec<u16> {
        text.encode_utf16().chain(std::iter::once(0)).collect()
    }

    const fn rgb_to_colorref(red: u8, green: u8, blue: u8) -> u32 {
        (red as u32) | ((green as u32) << 8) | ((blue as u32) << 16)
    }

    fn set_bool_attribute(hwnd: HWND, attribute: i32, value: bool) {
        let value: i32 = i32::from(value);
        set_attribute(hwnd, attribute, &value);
    }

    fn set_color_attribute(hwnd: HWND, attribute: i32, color: u32) {
        set_attribute(hwnd, attribute, &color);
    }

    fn set_attribute<T>(hwnd: HWND, attribute: i32, value: &T) {
        let value_size = size_of::<T>()
            .try_into()
            .expect("DWM attribute size fits in u32");

        unsafe {
            let _ = DwmSetWindowAttribute(
                hwnd,
                attribute as u32,
                std::ptr::from_ref(value).cast(),
                value_size,
            );
        }
    }
}

#[cfg(target_os = "windows")]
pub fn apply_when_ready(title: &'static str) {
    windows_title_bar::apply_when_ready(title);
}

#[cfg(not(target_os = "windows"))]
pub fn apply_when_ready(_title: &'static str) {}
