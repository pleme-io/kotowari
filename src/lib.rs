//! Kotowari (理) — LSP client, diagnostics UI, and code intelligence for Neovim.
//!
//! A Rust-native replacement for lspconfig + lspsaga + trouble. Configures
//! and starts LSP servers automatically based on filetype, sets up standard
//! keymaps on attach, and provides a trouble-like diagnostics list.
//!
//! Part of the blnvim-ng distribution — built with [`nvim-oxi`] and the
//! [`tane`] SDK.

pub mod diagnostics;
pub mod keymaps;
pub mod servers;

use nvim_oxi as oxi;
use nvim_oxi::api;

/// Plugin entry point.
///
/// Called once when Neovim loads the kotowari shared library. Sets up:
/// 1. `LspAttach` autocmd for keymap registration
/// 2. `FileType` autocmd for auto-starting language servers
/// 3. `:KotowariDiagnostics` user command
/// 4. Diagnostic highlight groups
#[oxi::plugin]
fn kotowari() -> oxi::Result<()> {
    // Set up the LspAttach autocmd that registers keymaps.
    keymaps::setup_on_attach().map_err(to_oxi_err)?;

    // Set up the FileType autocmd that auto-starts servers.
    setup_filetype_autocmd().map_err(to_oxi_err)?;

    // Register the diagnostics command and highlights.
    diagnostics::setup_diagnostics_command().map_err(to_oxi_err)?;
    diagnostics::setup_highlights().map_err(to_oxi_err)?;

    Ok(())
}

/// Register a `FileType` autocmd that starts the appropriate LSP server
/// when a buffer with a known filetype is opened.
fn setup_filetype_autocmd() -> tane::Result<()> {
    use tane::autocmd::Autocmd;

    Autocmd::on(&["FileType"])
        .group("KotowariAutoStart")
        .pattern("*")
        .desc("Auto-start LSP server for known filetypes")
        .register(move |args| {
            // Get the filetype from the match string (set by the FileType event).
            let ft = args.r#match;

            // Check if we have a server for this filetype.
            if let Some(config) = servers::server_for_filetype(&ft) {
                if let Err(e) = keymaps::start_server(config) {
                    let msg = format!("[kotowari] failed to start {}: {e}", config.name);
                    api::err_writeln(&msg);
                }
            }

            Ok(false)
        })?;

    Ok(())
}

/// Convert a `tane::Error` into an `oxi::Error`.
fn to_oxi_err(e: tane::Error) -> oxi::Error {
    // tane::Error wraps oxi::Error internally; extract it when possible.
    match e {
        tane::Error::Oxi(inner) => inner,
        other => oxi::Error::Api(api::Error::Other(other.to_string())),
    }
}
