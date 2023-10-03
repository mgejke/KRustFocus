use windows::Win32::Foundation::HWND;

pub(crate) enum ControlFlow {
    Continue,
    Exit,
}

#[derive(Debug, Default)]
pub(crate) struct FocusableWindow {
    pub hwnd: HWND,
    pub name: String,
    pub x: i32,
}

#[derive(Debug, Default)]
pub(crate) struct TabStopWindows {
    pub windows: Vec<FocusableWindow>,
    pub active: FocusableWindow,
}
