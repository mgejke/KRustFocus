use windows::Win32::Foundation::HWND;

pub(crate) enum ControlFlow {
    Continue,
    Exit,
}

#[derive(Debug, Default)]
pub(crate) struct FocusableWindow {
    pub hwnd: HWND,
    pub name: String,
    pub x1: i32,
    pub avg: i32,
}

#[derive(Clone, Copy)]
pub(crate) enum ComparePosition {
    Left,
    Avg,
}

impl FocusableWindow {
    pub(crate) fn new(hwnd: HWND, name: String, x1: i32, x2: i32) -> Self {
        let avg = (x1 + x2) as f32 / 2.0;
        Self {
            hwnd,
            name,
            x1,
            avg: avg as i32,
        }
    }

    pub(crate) fn get_position(&self, compare: ComparePosition) -> i32 {
        match compare {
            ComparePosition::Left => self.x1,
            ComparePosition::Avg => self.avg,
        }
    }
}

pub(crate) struct TabStopWindows {
    pub windows: Vec<FocusableWindow>,
    pub active: FocusableWindow,
    pub window_filter: Box<dyn Fn(&str) -> bool>,
}

impl Default for TabStopWindows {
    fn default() -> Self {
        Self {
            windows: Default::default(),
            active: Default::default(),
            window_filter: Box::new(|str| !str.contains("Settings")),
        }
    }
}

impl std::fmt::Debug for TabStopWindows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabStopWindows")
            .field("windows", &self.windows)
            .field("active", &self.active)
            .finish()
    }
}
