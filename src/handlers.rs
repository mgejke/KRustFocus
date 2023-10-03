use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{
        EnumWindows, GetForegroundWindow, GetWindowInfo, GetWindowTextW, SetForegroundWindow,
        WINDOWINFO, WS_TABSTOP, WS_VISIBLE,
    },
};
use winvd::is_window_on_current_desktop;

use crate::models::{ControlFlow, FocusableWindow, TabStopWindows};

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

            let awindow = FocusableWindow {
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

pub(crate) fn handle_left() -> ControlFlow {
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
}

pub(crate) fn handle_right() -> ControlFlow {
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
}

pub(crate) fn handle_quit() -> ControlFlow {
    println!("Ctrl-Alt-Shift L was pressed - Exiting...");
    ControlFlow::Exit
}