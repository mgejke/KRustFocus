#![windows_subsystem = "windows"]

mod command_line;
mod handlers;
mod models;

use anyhow::Result;
use tray_item::{IconSource, TrayItem};
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

use crate::command_line::parse_args;
use crate::handlers::{left, quit, right};
use crate::models::ControlFlow;

fn main() -> Result<()> {
    let (sort_mode, mod_keys, left_key, right_key) = parse_args()?;

    let mut hkm = HotkeyManager::new();
    hkm.register(left_key, &mod_keys, move || left(sort_mode))?;
    hkm.register(right_key, &mod_keys, move || right(sort_mode))?;

    hkm.register(VKey::L, &[ModKey::Ctrl, ModKey::Alt, ModKey::Shift], quit)?;
    let handle = hkm.interrupt_handle();

    let mut tray = TrayItem::new("KRustFocus", IconSource::Resource("focus_tray"))?;
    tray.add_label("KRustFocus")?;
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
