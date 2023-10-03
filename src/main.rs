use anyhow::Result;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowInfo, GetWindowTextW, SetForegroundWindow,
    WINDOWINFO, WS_TABSTOP, WS_VISIBLE,
};
use windows::{Win32::Foundation::BOOL, Win32::Foundation::HWND, Win32::Foundation::LPARAM};
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

use winvd::is_window_on_current_desktop;

enum ControlFlow {
    Continue,
    Exit,
}

#[derive(Debug, Default)]
struct AWindow {
    hwnd: HWND,
    name: String,
    x: i32,
}

#[derive(Debug, Default)]
struct TabStopWindows {
    windows: Vec<AWindow>,
    active: AWindow,
}

fn get_windows() -> TabStopWindows {
    let mut windows = TabStopWindows::default();
    unsafe {
        EnumWindows(
            Some(enum_window),
            LPARAM(&mut windows as *mut TabStopWindows as isize),
        )
    };
    windows
}

extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
    unsafe {
        let windows = &mut *(lparam.0 as *mut TabStopWindows);

        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };

        GetWindowInfo(window, &mut info).unwrap();
        if !text.is_empty()
            && info.dwStyle & WS_VISIBLE.0 > 0
            && info.dwStyle & WS_TABSTOP.0 > 0
            && is_window_on_current_desktop(window).unwrap()
        {
            let foreground = GetForegroundWindow();

            let awindow = AWindow {
                hwnd: window,
                x: info.rcWindow.left,
                name: text,
            };
            if foreground == window {
                windows.active = awindow
            } else {
                windows.windows.push(awindow)
            }
        }
        true.into()
    }
}

fn main() -> Result<()> {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::Left, &[ModKey::Ctrl, ModKey::Alt], || {
        let mut windows = get_windows();
        windows.windows.sort_by(|v1, v2| v2.x.cmp(&v1.x));

        if let Some(selected) = windows
            .windows
            .iter()
            .find(|window| window.x < windows.active.x)
        {
            println!("Switching to {}", selected.name);
            unsafe {
                SetForegroundWindow(selected.hwnd);
            }
        }
        ControlFlow::Continue
    })?;

    hkm.register(VKey::Right, &[ModKey::Ctrl, ModKey::Alt], || {
        let mut windows = get_windows();
        windows.windows.sort_by(|v1, v2| v1.x.cmp(&v2.x));

        if let Some(selected) = windows
            .windows
            .iter()
            .find(|window| window.x > windows.active.x)
        {
            println!("Switching to {}", selected.name);
            unsafe {
                SetForegroundWindow(selected.hwnd);
            }
        }
        ControlFlow::Continue
    })?;

    hkm.register(VKey::L, &[ModKey::Ctrl, ModKey::Alt, ModKey::Shift], || {
        println!("Ctrl-Alt-Shift L was pressed - Exiting...");
        ControlFlow::Exit
    })?;

    loop {
        match hkm.handle_hotkey() {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }
    Ok(())
}
