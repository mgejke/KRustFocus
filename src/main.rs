// #![windows_subsystem = "windows"]

mod handlers;
mod models;

use anyhow::Result;
use handlers::{left, quit, right};
use models::ControlFlow;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

use tray_item::{IconSource, TrayItem};

fn main() -> Result<()> {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::Left, &[ModKey::Ctrl, ModKey::Alt], move || {
        left(models::ComparePosition::Avg)
    })?;
    hkm.register(VKey::Right, &[ModKey::Ctrl, ModKey::Alt], move || {
        right(models::ComparePosition::Avg)
    })?;

    hkm.register(VKey::L, &[ModKey::Ctrl, ModKey::Alt, ModKey::Shift], quit)?;
    let handle = hkm.interrupt_handle();

    let mut tray = TrayItem::new("Window Focuser", IconSource::Resource("focus_tray"))?;
    tray.add_label("Window Focuser")?;
    tray.inner_mut().add_separator()?;
    tray.add_menu_item("Quit", move || {
        handle.interrupt();
    })?;

    loop {
        match hkm.handle_hotkey() {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }
    Ok(())
}
