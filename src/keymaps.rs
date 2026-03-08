//! LSP keymap registration.
//!
//! Sets up standard LSP keymaps when a language server attaches to a buffer.
//! Uses Neovim's built-in `vim.lsp.buf.*` functions via `<Cmd>lua ...<CR>`
//! mappings.

use nvim_oxi::api;
use nvim_oxi::api::Buffer;
use nvim_oxi::api::opts::SetKeymapOpts;
use nvim_oxi::api::types::Mode;

/// Register all standard LSP keymaps for the given buffer.
///
/// These keymaps are buffer-local so they only activate when the LSP client
/// is attached. Each keymap calls the corresponding `vim.lsp.buf.*` function.
pub fn register_lsp_keymaps(buffer: &mut Buffer) -> tane::Result<()> {
    let keymaps: &[(&str, &str, &str)] = &[
        ("gd", "lua vim.lsp.buf.definition()", "Go to definition"),
        ("gD", "lua vim.lsp.buf.declaration()", "Go to declaration"),
        (
            "gi",
            "lua vim.lsp.buf.implementation()",
            "Go to implementation",
        ),
        ("gr", "lua vim.lsp.buf.references()", "Show references"),
        ("K", "lua vim.lsp.buf.hover()", "Hover documentation"),
        (
            "<C-k>",
            "lua vim.lsp.buf.signature_help()",
            "Signature help",
        ),
        (
            "<leader>ca",
            "lua vim.lsp.buf.code_action()",
            "Code action",
        ),
        (
            "<leader>rn",
            "lua vim.lsp.buf.rename()",
            "Rename symbol",
        ),
        (
            "<leader>f",
            "lua vim.lsp.buf.format({ async = true })",
            "Format buffer",
        ),
        (
            "<leader>td",
            "lua vim.lsp.buf.type_definition()",
            "Type definition",
        ),
    ];

    for &(lhs, rhs, desc) in keymaps {
        let rhs_cmd = format!("<Cmd>{rhs}<CR>");
        set_buffer_keymap(buffer, lhs, &rhs_cmd, desc)?;
    }

    // Diagnostic navigation keymaps (not buffer-local to vim.lsp.buf, but
    // closely related and expected by users).
    let diag_keymaps: &[(&str, &str, &str)] = &[
        (
            "[d",
            "lua vim.diagnostic.goto_prev()",
            "Previous diagnostic",
        ),
        (
            "]d",
            "lua vim.diagnostic.goto_next()",
            "Next diagnostic",
        ),
        (
            "<leader>e",
            "lua vim.diagnostic.open_float()",
            "Show diagnostic float",
        ),
        (
            "<leader>q",
            "KotowariDiagnostics",
            "Toggle diagnostics list",
        ),
    ];

    for &(lhs, rhs, desc) in diag_keymaps {
        let rhs_cmd = format!("<Cmd>{rhs}<CR>");
        set_buffer_keymap(buffer, lhs, &rhs_cmd, desc)?;
    }

    Ok(())
}

/// Set a single buffer-local normal-mode keymap.
fn set_buffer_keymap(
    buffer: &mut Buffer,
    lhs: &str,
    rhs: &str,
    desc: &str,
) -> tane::Result<()> {
    let opts = SetKeymapOpts::builder()
        .silent(true)
        .desc(desc)
        .build();
    buffer.set_keymap(Mode::Normal, lhs, rhs, &opts)?;
    Ok(())
}

/// Generate the Lua script that starts a language server via `vim.lsp.start`.
///
/// This produces a self-contained Lua string that can be executed with
/// `api::exec` to start the server for the current buffer.
#[must_use]
pub fn lsp_start_lua(
    name: &str,
    cmd: &[&str],
    root_markers: &[&str],
    init_options: Option<&str>,
    settings: Option<&str>,
) -> String {
    // Build the cmd table.
    let cmd_lua: String = cmd
        .iter()
        .map(|c| format!("'{c}'"))
        .collect::<Vec<_>>()
        .join(", ");

    // Build the root_markers table.
    let markers_lua: String = root_markers
        .iter()
        .map(|m| format!("'{m}'"))
        .collect::<Vec<_>>()
        .join(", ");

    let init_opts_lua = match init_options {
        Some(json) => format!("vim.json.decode([[{json}]])"),
        None => "vim.empty_dict()".to_string(),
    };

    let settings_lua = match settings {
        Some(json) => format!("vim.json.decode([[{json}]])"),
        None => "vim.empty_dict()".to_string(),
    };

    format!(
        r#"
local root_dir = vim.fs.root(0, {{{markers_lua}}}) or vim.fn.getcwd()
vim.lsp.start({{
  name = '{name}',
  cmd = {{{cmd_lua}}},
  root_dir = root_dir,
  init_options = {init_opts_lua},
  settings = {settings_lua},
}})
"#,
    )
}

/// Start a language server for the current buffer using the given server config.
pub fn start_server(config: &crate::servers::ServerConfig) -> tane::Result<()> {
    let lua = lsp_start_lua(
        config.name,
        config.cmd,
        config.root_markers,
        config.init_options,
        config.settings,
    );
    // Use `api::exec2` to run the multi-line Lua script via heredoc.
    let vim_script = format!("lua << EOF\n{lua}\nEOF");
    let opts = api::opts::ExecOpts::default();
    api::exec2(&vim_script, &opts)?;
    Ok(())
}

/// Set up an `LspAttach` autocmd that registers keymaps when any LSP client
/// attaches to a buffer.
pub fn setup_on_attach() -> tane::Result<()> {
    use tane::autocmd::Autocmd;

    Autocmd::on(&["LspAttach"])
        .group("KotowariLspAttach")
        .desc("Register LSP keymaps on attach")
        .register(move |args| {
            let mut buffer = args.buffer;
            // Register keymaps for this buffer. Errors are logged but not fatal.
            if let Err(e) = register_lsp_keymaps(&mut buffer) {
                let msg = format!("[kotowari] keymap setup failed: {e}");
                api::err_writeln(&msg);
            }
            Ok(false)
        })?;

    Ok(())
}
