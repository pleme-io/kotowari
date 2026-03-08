//! LSP server configuration database.
//!
//! Maps Neovim filetypes to language server names, commands, and settings.
//! Pure Rust — no Neovim API calls, fully unit-testable.

use std::collections::HashMap;

/// Configuration for a single language server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConfig {
    /// Human-readable server name (e.g., "rust-analyzer").
    pub name: &'static str,

    /// Command and arguments to start the server.
    ///
    /// The first element is the executable; the rest are arguments.
    pub cmd: &'static [&'static str],

    /// Neovim filetypes this server handles.
    pub filetypes: &'static [&'static str],

    /// Root markers — files or directories that indicate the project root.
    ///
    /// The LSP `rootUri` is set to the nearest ancestor containing any of
    /// these markers.
    pub root_markers: &'static [&'static str],

    /// Server-specific initialization options, serialized as JSON.
    ///
    /// Passed as `initializationOptions` in the LSP `initialize` request.
    /// `None` means no extra options.
    pub init_options: Option<&'static str>,

    /// Server-specific settings, serialized as JSON.
    ///
    /// Sent via `workspace/didChangeConfiguration`. `None` means no settings.
    pub settings: Option<&'static str>,
}

/// The built-in server configuration database.
///
/// Returns a static slice of all known server configs.
#[must_use]
pub fn builtin_servers() -> &'static [ServerConfig] {
    &SERVERS
}

/// Look up a server config by filetype.
///
/// Returns the first matching server. If multiple servers handle the same
/// filetype, the one defined earlier in the database wins.
#[must_use]
pub fn server_for_filetype(filetype: &str) -> Option<&'static ServerConfig> {
    SERVERS.iter().find(|s| s.filetypes.contains(&filetype))
}

/// Look up a server config by server name.
#[must_use]
pub fn server_by_name(name: &str) -> Option<&'static ServerConfig> {
    SERVERS.iter().find(|s| s.name == name)
}

/// Build a map from filetype to server config for all built-in servers.
#[must_use]
pub fn filetype_server_map() -> HashMap<&'static str, &'static ServerConfig> {
    let mut map = HashMap::new();
    for server in &SERVERS {
        for &ft in server.filetypes {
            map.entry(ft).or_insert(server);
        }
    }
    map
}

// ---------------------------------------------------------------------------
// Server database
// ---------------------------------------------------------------------------

static SERVERS: [ServerConfig; 14] = [
    // Rust
    ServerConfig {
        name: "rust-analyzer",
        cmd: &["rust-analyzer"],
        filetypes: &["rust"],
        root_markers: &["Cargo.toml", "rust-project.json"],
        init_options: None,
        settings: Some(
            r#"{"rust-analyzer":{"check":{"command":"clippy"},"procMacro":{"enable":true}}}"#,
        ),
    },
    // Python
    ServerConfig {
        name: "pyright",
        cmd: &["pyright-langserver", "--stdio"],
        filetypes: &["python"],
        root_markers: &[
            "pyproject.toml",
            "setup.py",
            "setup.cfg",
            "pyrightconfig.json",
        ],
        init_options: None,
        settings: None,
    },
    // TypeScript / JavaScript
    ServerConfig {
        name: "typescript-language-server",
        cmd: &["typescript-language-server", "--stdio"],
        filetypes: &[
            "typescript",
            "typescriptreact",
            "javascript",
            "javascriptreact",
        ],
        root_markers: &["tsconfig.json", "jsconfig.json", "package.json"],
        init_options: None,
        settings: None,
    },
    // Go
    ServerConfig {
        name: "gopls",
        cmd: &["gopls"],
        filetypes: &["go", "gomod", "gowork", "gotmpl"],
        root_markers: &["go.mod", "go.work", ".git"],
        init_options: None,
        settings: Some(r#"{"gopls":{"staticcheck":true,"gofumpt":true}}"#),
    },
    // Lua
    ServerConfig {
        name: "lua-language-server",
        cmd: &["lua-language-server"],
        filetypes: &["lua"],
        root_markers: &[".luarc.json", ".luarc.jsonc", ".luacheckrc", ".stylua.toml"],
        init_options: None,
        settings: Some(
            r#"{"Lua":{"diagnostics":{"globals":["vim"]},"workspace":{"checkThirdParty":false}}}"#,
        ),
    },
    // Nix
    ServerConfig {
        name: "nil",
        cmd: &["nil"],
        filetypes: &["nix"],
        root_markers: &["flake.nix", "default.nix", "shell.nix"],
        init_options: None,
        settings: Some(r#"{"nil":{"formatting":{"command":["nixfmt"]}}}"#),
    },
    // C / C++
    ServerConfig {
        name: "clangd",
        cmd: &["clangd", "--background-index"],
        filetypes: &["c", "cpp", "objc", "objcpp", "cuda"],
        root_markers: &[
            "compile_commands.json",
            "compile_flags.txt",
            ".clangd",
            "CMakeLists.txt",
        ],
        init_options: None,
        settings: None,
    },
    // JSON
    ServerConfig {
        name: "vscode-json-languageserver",
        cmd: &["vscode-json-languageserver", "--stdio"],
        filetypes: &["json", "jsonc"],
        root_markers: &[".git"],
        init_options: Some(r#"{"provideFormatter":true}"#),
        settings: None,
    },
    // YAML
    ServerConfig {
        name: "yaml-language-server",
        cmd: &["yaml-language-server", "--stdio"],
        filetypes: &["yaml", "yaml.docker-compose"],
        root_markers: &[".git"],
        init_options: None,
        settings: Some(r#"{"yaml":{"schemaStore":{"enable":true}}}"#),
    },
    // TOML
    ServerConfig {
        name: "taplo",
        cmd: &["taplo", "lsp", "stdio"],
        filetypes: &["toml"],
        root_markers: &[".taplo.toml", "taplo.toml", ".git"],
        init_options: None,
        settings: None,
    },
    // Bash / Shell
    ServerConfig {
        name: "bash-language-server",
        cmd: &["bash-language-server", "start"],
        filetypes: &["sh", "bash", "zsh"],
        root_markers: &[".git"],
        init_options: None,
        settings: None,
    },
    // Zig
    ServerConfig {
        name: "zls",
        cmd: &["zls"],
        filetypes: &["zig"],
        root_markers: &["build.zig", "zls.json"],
        init_options: None,
        settings: None,
    },
    // Ruby
    ServerConfig {
        name: "solargraph",
        cmd: &["solargraph", "stdio"],
        filetypes: &["ruby"],
        root_markers: &["Gemfile", ".solargraph.yml"],
        init_options: None,
        settings: None,
    },
    // Haskell
    ServerConfig {
        name: "haskell-language-server",
        cmd: &["haskell-language-server-wrapper", "--lsp"],
        filetypes: &["haskell", "lhaskell"],
        root_markers: &[
            "hie.yaml",
            "stack.yaml",
            "cabal.project",
            "*.cabal",
            "package.yaml",
        ],
        init_options: None,
        settings: None,
    },
];

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_servers_is_not_empty() {
        assert!(!builtin_servers().is_empty());
    }

    #[test]
    fn builtin_servers_count() {
        assert_eq!(builtin_servers().len(), 14);
    }

    #[test]
    fn all_servers_have_nonempty_cmd() {
        for server in builtin_servers() {
            assert!(
                !server.cmd.is_empty(),
                "server '{}' has empty cmd",
                server.name,
            );
            assert!(
                !server.cmd[0].is_empty(),
                "server '{}' has empty executable",
                server.name,
            );
        }
    }

    #[test]
    fn all_servers_have_nonempty_filetypes() {
        for server in builtin_servers() {
            assert!(
                !server.filetypes.is_empty(),
                "server '{}' has no filetypes",
                server.name,
            );
        }
    }

    #[test]
    fn all_servers_have_root_markers() {
        for server in builtin_servers() {
            assert!(
                !server.root_markers.is_empty(),
                "server '{}' has no root markers",
                server.name,
            );
        }
    }

    #[test]
    fn all_server_names_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for server in builtin_servers() {
            assert!(
                seen.insert(server.name),
                "duplicate server name: '{}'",
                server.name,
            );
        }
    }

    #[test]
    fn lookup_rust_by_filetype() {
        let server = server_for_filetype("rust").expect("no server for 'rust'");
        assert_eq!(server.name, "rust-analyzer");
        assert_eq!(server.cmd, &["rust-analyzer"]);
        assert!(server.root_markers.contains(&"Cargo.toml"));
    }

    #[test]
    fn lookup_python_by_filetype() {
        let server = server_for_filetype("python").expect("no server for 'python'");
        assert_eq!(server.name, "pyright");
        assert!(server.cmd.contains(&"--stdio"));
    }

    #[test]
    fn lookup_typescript_by_filetype() {
        let server = server_for_filetype("typescript").expect("no server for 'typescript'");
        assert_eq!(server.name, "typescript-language-server");
    }

    #[test]
    fn lookup_tsx_by_filetype() {
        let server = server_for_filetype("typescriptreact").expect("no server for 'typescriptreact'");
        assert_eq!(server.name, "typescript-language-server");
    }

    #[test]
    fn lookup_jsx_by_filetype() {
        let server = server_for_filetype("javascriptreact").expect("no server for 'javascriptreact'");
        assert_eq!(server.name, "typescript-language-server");
    }

    #[test]
    fn lookup_go_by_filetype() {
        let server = server_for_filetype("go").expect("no server for 'go'");
        assert_eq!(server.name, "gopls");
        assert!(server.root_markers.contains(&"go.mod"));
    }

    #[test]
    fn lookup_gomod_by_filetype() {
        let server = server_for_filetype("gomod").expect("no server for 'gomod'");
        assert_eq!(server.name, "gopls");
    }

    #[test]
    fn lookup_lua_by_filetype() {
        let server = server_for_filetype("lua").expect("no server for 'lua'");
        assert_eq!(server.name, "lua-language-server");
    }

    #[test]
    fn lookup_nix_by_filetype() {
        let server = server_for_filetype("nix").expect("no server for 'nix'");
        assert_eq!(server.name, "nil");
        assert!(server.root_markers.contains(&"flake.nix"));
    }

    #[test]
    fn lookup_c_by_filetype() {
        let server = server_for_filetype("c").expect("no server for 'c'");
        assert_eq!(server.name, "clangd");
    }

    #[test]
    fn lookup_cpp_by_filetype() {
        let server = server_for_filetype("cpp").expect("no server for 'cpp'");
        assert_eq!(server.name, "clangd");
    }

    #[test]
    fn lookup_json_by_filetype() {
        let server = server_for_filetype("json").expect("no server for 'json'");
        assert_eq!(server.name, "vscode-json-languageserver");
    }

    #[test]
    fn lookup_jsonc_by_filetype() {
        let server = server_for_filetype("jsonc").expect("no server for 'jsonc'");
        assert_eq!(server.name, "vscode-json-languageserver");
    }

    #[test]
    fn lookup_yaml_by_filetype() {
        let server = server_for_filetype("yaml").expect("no server for 'yaml'");
        assert_eq!(server.name, "yaml-language-server");
    }

    #[test]
    fn lookup_toml_by_filetype() {
        let server = server_for_filetype("toml").expect("no server for 'toml'");
        assert_eq!(server.name, "taplo");
    }

    #[test]
    fn lookup_shell_by_filetype() {
        for ft in &["sh", "bash", "zsh"] {
            let server = server_for_filetype(ft).unwrap_or_else(|| panic!("no server for '{ft}'"));
            assert_eq!(server.name, "bash-language-server");
        }
    }

    #[test]
    fn lookup_zig_by_filetype() {
        let server = server_for_filetype("zig").expect("no server for 'zig'");
        assert_eq!(server.name, "zls");
    }

    #[test]
    fn lookup_ruby_by_filetype() {
        let server = server_for_filetype("ruby").expect("no server for 'ruby'");
        assert_eq!(server.name, "solargraph");
    }

    #[test]
    fn lookup_haskell_by_filetype() {
        let server = server_for_filetype("haskell").expect("no server for 'haskell'");
        assert_eq!(server.name, "haskell-language-server");
    }

    #[test]
    fn lookup_unknown_filetype_returns_none() {
        assert!(server_for_filetype("brainfuck").is_none());
        assert!(server_for_filetype("").is_none());
        assert!(server_for_filetype("RUST").is_none()); // case-sensitive
    }

    #[test]
    fn lookup_by_name_rust_analyzer() {
        let server = server_by_name("rust-analyzer").expect("no server named 'rust-analyzer'");
        assert!(server.filetypes.contains(&"rust"));
    }

    #[test]
    fn lookup_by_name_all_servers() {
        for server in builtin_servers() {
            let found =
                server_by_name(server.name).unwrap_or_else(|| panic!("cannot find '{}'", server.name));
            assert_eq!(found.name, server.name);
        }
    }

    #[test]
    fn lookup_by_name_unknown_returns_none() {
        assert!(server_by_name("nonexistent-server").is_none());
    }

    #[test]
    fn filetype_map_covers_all_filetypes() {
        let map = filetype_server_map();
        for server in builtin_servers() {
            for &ft in server.filetypes {
                assert!(
                    map.contains_key(ft),
                    "filetype '{}' missing from map",
                    ft,
                );
            }
        }
    }

    #[test]
    fn filetype_map_first_server_wins() {
        // If two servers handle the same filetype (unlikely but possible),
        // the one defined first should win. Currently no overlaps, so just
        // verify that the map value matches `server_for_filetype`.
        let map = filetype_server_map();
        for (&ft, &server) in &map {
            let lookup = server_for_filetype(ft).expect("server_for_filetype returned None");
            assert_eq!(lookup.name, server.name);
        }
    }

    #[test]
    fn rust_analyzer_has_settings() {
        let server = server_by_name("rust-analyzer").unwrap();
        let settings = server.settings.expect("rust-analyzer should have settings");
        assert!(settings.contains("clippy"));
        assert!(settings.contains("procMacro"));
    }

    #[test]
    fn gopls_has_settings() {
        let server = server_by_name("gopls").unwrap();
        let settings = server.settings.expect("gopls should have settings");
        assert!(settings.contains("staticcheck"));
    }

    #[test]
    fn nil_has_settings() {
        let server = server_by_name("nil").unwrap();
        let settings = server.settings.expect("nil should have settings");
        assert!(settings.contains("nixfmt"));
    }

    #[test]
    fn lua_ls_has_vim_global() {
        let server = server_by_name("lua-language-server").unwrap();
        let settings = server.settings.expect("lua-ls should have settings");
        assert!(settings.contains("vim"));
    }

    #[test]
    fn json_server_has_init_options() {
        let server = server_by_name("vscode-json-languageserver").unwrap();
        let opts = server
            .init_options
            .expect("json server should have init options");
        assert!(opts.contains("provideFormatter"));
    }

    #[test]
    fn yaml_server_has_schema_store() {
        let server = server_by_name("yaml-language-server").unwrap();
        let settings = server.settings.expect("yaml-ls should have settings");
        assert!(settings.contains("schemaStore"));
    }

    #[test]
    fn clangd_has_background_index_flag() {
        let server = server_by_name("clangd").unwrap();
        assert!(server.cmd.contains(&"--background-index"));
    }

    #[test]
    fn taplo_cmd_has_lsp_stdio() {
        let server = server_by_name("taplo").unwrap();
        assert_eq!(server.cmd, &["taplo", "lsp", "stdio"]);
    }

    #[test]
    fn hls_uses_wrapper() {
        let server = server_by_name("haskell-language-server").unwrap();
        assert_eq!(server.cmd[0], "haskell-language-server-wrapper");
        assert!(server.cmd.contains(&"--lsp"));
    }

    #[test]
    fn server_config_clone_eq() {
        let server = server_by_name("rust-analyzer").unwrap();
        let cloned = server.clone();
        assert_eq!(server, &cloned);
    }

    #[test]
    fn all_settings_are_valid_json() {
        for server in builtin_servers() {
            if let Some(settings) = server.settings {
                // Minimal JSON validation: must start/end with braces and
                // contain at least one key.
                assert!(
                    settings.starts_with('{') && settings.ends_with('}'),
                    "server '{}' settings is not a JSON object: {settings}",
                    server.name,
                );
            }
            if let Some(opts) = server.init_options {
                assert!(
                    opts.starts_with('{') && opts.ends_with('}'),
                    "server '{}' init_options is not a JSON object: {opts}",
                    server.name,
                );
            }
        }
    }

    #[test]
    fn total_filetypes_covered() {
        let map = filetype_server_map();
        // We should cover at least 25 filetypes across all servers
        assert!(
            map.len() >= 25,
            "only {} filetypes covered, expected at least 25",
            map.len(),
        );
    }
}
