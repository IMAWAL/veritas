#![recursion_limit = "256"]
#![allow(static_mut_refs)]
#![feature(windows_process_extensions_show_window)]

mod battle;
mod entry;
mod export;
mod kreide;
mod logging;
mod models;
mod server;
mod subscribers;

use anyhow::{Context, Result};
use std::sync::LazyLock;
use tokio::runtime::Runtime;
use windows::{Win32::System::LibraryLoader::GetModuleHandleW, core::PCWSTR};

fn get_module_handle(name: PCWSTR) -> Result<usize> {
    unsafe {
        GetModuleHandleW(name)
            .map(|v| v.0 as usize)
            .context("Failed to get module handle")
    }
}

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Runtime::new().unwrap_or_else(|e| {
        log::error!("{e}");
        panic!("{e}");
    })
});
