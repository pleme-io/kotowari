//! Trouble-like diagnostic list window.
//!
//! Provides a `:KotowariDiagnostics` command that opens a split with
//! all diagnostics from attached LSP servers, formatted for quick
//! navigation.

use nvim_oxi::api;

/// The Lua script that collects diagnostics and populates the quickfix list.
///
/// We use Neovim's built-in `vim.diagnostic.setqflist()` to leverage the
/// native quickfix infrastructure, then open it in a bottom split. This
/// gives us jump-to-location, filtering, and persistence for free.
const DIAGNOSTICS_LUA: &str = r#"
vim.diagnostic.setqflist({ open = false })
local qflist = vim.fn.getqflist()
if #qflist == 0 then
  vim.notify('[kotowari] No diagnostics', vim.log.levels.INFO)
  return
end

-- Format the quickfix list with severity icons.
local icons = { 'E', 'W', 'I', 'H' }
for _, item in ipairs(qflist) do
  local sev = item.type or 'E'
  if type(sev) == 'number' then
    sev = icons[sev] or 'E'
  end
  item.type = sev
end
vim.fn.setqflist(qflist, 'r')

-- Open the quickfix window at the bottom, 10 lines tall.
vim.cmd('botright copen 10')

-- Set buffer-local options for the qf window.
local qf_buf = vim.api.nvim_get_current_buf()
vim.api.nvim_set_option_value('wrap', false, { buf = qf_buf })
"#;

/// Register the `:KotowariDiagnostics` user command.
pub fn setup_diagnostics_command() -> tane::Result<()> {
    use tane::usercmd::UserCommand;

    UserCommand::new("KotowariDiagnostics")
        .desc("Show LSP diagnostics in a trouble-like list")
        .bar()
        .register(|_args| {
            if let Err(e) = show_diagnostics() {
                let msg = format!("[kotowari] diagnostics error: {e}");
                api::err_writeln(&msg);
            }
            Ok(())
        })?;

    Ok(())
}

/// Open the diagnostics list.
fn show_diagnostics() -> tane::Result<()> {
    let vim_script = format!("lua << EOF\n{DIAGNOSTICS_LUA}\nEOF");
    let opts = api::opts::ExecOpts::default();
    api::exec2(&vim_script, &opts)?;
    Ok(())
}

/// Register highlight groups used by the diagnostics display.
pub fn setup_highlights() -> tane::Result<()> {
    use tane::highlight::Highlight;

    Highlight::new("KotowariDiagError")
        .link("DiagnosticError")
        .apply()?;
    Highlight::new("KotowariDiagWarn")
        .link("DiagnosticWarn")
        .apply()?;
    Highlight::new("KotowariDiagInfo")
        .link("DiagnosticInfo")
        .apply()?;
    Highlight::new("KotowariDiagHint")
        .link("DiagnosticHint")
        .apply()?;

    Ok(())
}
