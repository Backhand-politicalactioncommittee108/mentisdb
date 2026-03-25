use crate::integrations::{
    IntegrationFileFormat, IntegrationKind, IntegrationPathTarget, IntegrationSpec,
};
use crate::paths::{HostPlatform, PathEnvironment};
use std::path::PathBuf;

pub(super) fn specs(env: &PathEnvironment) -> Vec<IntegrationSpec> {
    let platform = HostPlatform::Macos;
    let home = user_root(env, platform);
    let app_support = env
        .config_root_for(platform)
        .unwrap_or_else(|| home.join("Library").join("Application Support"));
    let xdg_config = home.join(".config");

    vec![
        IntegrationSpec {
            integration: IntegrationKind::Codex,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".codex").join("config.toml"),
                "Primary Codex MCP and CLI config",
                IntegrationFileFormat::Toml,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                home.join(".codex"),
                "Codex home directory",
            )],
            companion_targets: vec![],
            notes: vec!["macOS first-class target: Codex config lives under ~/.codex.".into()],
        },
        IntegrationSpec {
            integration: IntegrationKind::ClaudeCode,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".claude").join("mcp").join("mentisdb.json"),
                "User-level Claude Code MCP entry",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![
                IntegrationPathTarget::directory(
                    home.join(".claude"),
                    "Claude Code home directory",
                ),
                IntegrationPathTarget::file(
                    home.join(".claude.json"),
                    "Claude Code global state file",
                    IntegrationFileFormat::Json,
                ),
            ],
            companion_targets: vec![IntegrationPathTarget::file(
                home.join(".claude").join("settings.json"),
                "General Claude Code settings",
                IntegrationFileFormat::Json,
            )],
            notes: vec![
                "Use ~/.claude/mcp/mentisdb.json for a stable user-level MCP entry."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::GeminiCli,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".gemini").join("settings.json"),
                "Primary Gemini CLI settings",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                home.join(".gemini"),
                "Gemini CLI home directory",
            )],
            companion_targets: vec![IntegrationPathTarget::file(
                home.join(".gemini").join("system.md"),
                "Optional Gemini system prompt file",
                IntegrationFileFormat::Markdown,
            )],
            notes: vec![
                "Gemini commonly stores settings.json in ~/.gemini; system.md is optional and should not be required for detection."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::OpenCode,
            platform,
            config_target: IntegrationPathTarget::file(
                xdg_config.join("opencode").join("opencode.json"),
                "Primary OpenCode settings",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                xdg_config.join("opencode"),
                "OpenCode config directory",
            )],
            companion_targets: vec![],
            notes: vec![
                "OpenCode uses the XDG-style ~/.config/opencode directory on macOS.".into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::Qwen,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".qwen").join("settings.json"),
                "Primary Qwen settings",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                home.join(".qwen"),
                "Qwen home directory",
            )],
            companion_targets: vec![],
            notes: vec!["Qwen is detected from ~/.qwen and configured via settings.json.".into()],
        },
        IntegrationSpec {
            integration: IntegrationKind::CopilotCli,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".copilot").join("mcp-config.json"),
                "GitHub Copilot CLI MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                home.join(".copilot"),
                "GitHub Copilot CLI home directory",
            )],
            companion_targets: vec![],
            notes: vec![
                "GitHub Copilot CLI stores custom MCP servers in ~/.copilot/mcp-config.json."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::VsCodeCopilot,
            platform,
            config_target: IntegrationPathTarget::file(
                app_support.join("Code").join("User").join("mcp.json"),
                "VS Code MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                app_support.join("Code").join("User"),
                "VS Code user settings directory",
            )],
            companion_targets: vec![IntegrationPathTarget::file(
                app_support.join("Code").join("User").join("settings.json"),
                "Existing VS Code user settings",
                IntegrationFileFormat::Json,
            )],
            notes: vec![
                "Treat the Code/User directory as the installation signal even when mcp.json is absent.".into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::ClaudeDesktop,
            platform,
            config_target: IntegrationPathTarget::file(
                app_support
                    .join("Claude")
                    .join("claude_desktop_config.json"),
                "Claude Desktop MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                app_support.join("Claude"),
                "Claude Desktop application support directory",
            )],
            companion_targets: vec![],
            notes: vec![
                "Claude Desktop exposes MCP servers through claude_desktop_config.json.".into(),
            ],
        },
    ]
}

fn user_root(env: &PathEnvironment, platform: HostPlatform) -> PathBuf {
    env.home_dir_for(platform)
        .or_else(|| env.current_dir.clone())
        .unwrap_or_else(|| PathBuf::from("."))
}
