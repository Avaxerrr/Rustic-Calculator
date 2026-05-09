#[cfg(target_os = "windows")]
mod windows_clipboard {
    const CF_UNICODETEXT: u32 = 13;
    use windows_sys::Win32::System::DataExchange::{
        CloseClipboard, GetClipboardData, IsClipboardFormatAvailable, OpenClipboard,
    };
    use windows_sys::Win32::System::Memory::{GlobalLock, GlobalUnlock};

    pub fn text() -> Option<String> {
        unsafe {
            if IsClipboardFormatAvailable(CF_UNICODETEXT) == 0
                || OpenClipboard(std::ptr::null_mut()) == 0
            {
                return None;
            }

            let result = read_text();
            CloseClipboard();
            result
        }
    }

    unsafe fn read_text() -> Option<String> {
        let handle = unsafe { GetClipboardData(CF_UNICODETEXT) };
        if handle.is_null() {
            return None;
        }

        let data = unsafe { GlobalLock(handle) }.cast::<u16>();
        if data.is_null() {
            return None;
        }

        let mut len = 0;
        while unsafe { *data.add(len) } != 0 {
            len += 1;
        }

        let text = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(data, len) });
        unsafe {
            GlobalUnlock(handle);
        }
        Some(text)
    }
}

#[cfg(target_os = "windows")]
pub fn text() -> Option<String> {
    windows_clipboard::text()
}

#[cfg(not(target_os = "windows"))]
pub fn text() -> Option<String> {
    None
}
