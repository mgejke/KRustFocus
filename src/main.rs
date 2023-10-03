mod handlers;
mod models;

use anyhow::Result;
use handlers::{handle_left, handle_quit, handle_right};
use models::ControlFlow;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

fn main() -> Result<()> {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::Left, &[ModKey::Ctrl, ModKey::Alt], handle_left)?;
    hkm.register(VKey::Right, &[ModKey::Ctrl, ModKey::Alt], handle_right)?;
    hkm.register(
        VKey::L,
        &[ModKey::Ctrl, ModKey::Alt, ModKey::Shift],
        handle_quit,
    )?;

    loop {
        match hkm.handle_hotkey() {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }
    Ok(())
}
