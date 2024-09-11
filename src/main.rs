#![windows_subsystem = "windows"]

mod handlers;
mod models;

use anyhow::{anyhow, Result};
use handlers::{left, quit, right};
use models::{ComparePosition, ControlFlow};
use windows_hotkeys::keys::{ModKey, VKey};
use windows_hotkeys::{HotkeyManager, HotkeyManagerImpl};

use tray_item::{IconSource, TrayItem};

use clap::{Parser, ValueEnum};

/// Tool for focusing windows based on position
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use left or window avg for sorting
    #[arg(long, value_enum, default_value_t=SortMode::Avg)]
    mode: SortMode,

    #[arg(short, long, num_args = 1..=3, default_values=["CTRL", "ALT"])]
    mod_keys: Vec<String>,

    #[arg(short, long, default_value = "LEFT")]
    left: String,

    #[arg(short, long, default_value = "RIGHT")]
    right: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SortMode {
    Left,
    Avg,
}

impl From<SortMode> for ComparePosition {
    fn from(value: SortMode) -> Self {
        match value {
            SortMode::Left => ComparePosition::Left,
            SortMode::Avg => ComparePosition::Avg,
        }
    }
}

fn parse_args() -> Result<(ComparePosition, Vec<ModKey>, VKey, VKey)> {
    let args = Args::parse();
    let sort_mode = args.mode.into();

    let mod_keys: Vec<_> = args
        .mod_keys
        .iter()
        .map(|val| ModKey::from_keyname(val))
        .collect();
    let mod_keys = match mod_keys.into_iter().collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => {
            return Err(anyhow!(
                "Valid values for mod_keys are CTRL, ALT, WIN, SHIFT - {}",
                e
            ))
        }
    };
    let left_key = match VKey::from_keyname(&args.left) {
        Ok(vkey) => vkey,
        Err(e) => return Err(anyhow!("Could not assign key - {}", e)),
    };
    let right_key = match VKey::from_keyname(&args.right) {
        Ok(vkey) => vkey,
        Err(e) => return Err(anyhow!("Could not assign key - {}", e)),
    };
    Ok((sort_mode, mod_keys, left_key, right_key))
}

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
