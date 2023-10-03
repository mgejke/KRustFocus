mod handlers;
mod models;

use anyhow::Result;
use handlers::{left, quit, right};
use models::ControlFlow;
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

fn main() -> Result<()> {
    let mut hkm = HotkeyManager::new();

    hkm.register(VKey::Left, &[ModKey::Ctrl, ModKey::Alt], left)?;
    hkm.register(VKey::Right, &[ModKey::Ctrl, ModKey::Alt], right)?;
    hkm.register(VKey::L, &[ModKey::Ctrl, ModKey::Alt, ModKey::Shift], quit)?;

    loop {
        match hkm.handle_hotkey() {
            Some(ControlFlow::Exit) | None => break,
            _ => (),
        }
    }
    Ok(())
}
