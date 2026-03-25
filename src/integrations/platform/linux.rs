use crate::integrations::{
    IntegrationFileFormat, IntegrationKind, IntegrationPathTarget, IntegrationSpec,
};
use crate::paths::{HostPlatform, PathEnvironment};
use std::path::PathBuf;

pub(super) fn specs(platform: HostPlatform, env: &PathEnvironment) -> Vec<IntegrationSpec> {
    let home = user_root(env, platform);
    let config_root = env
        .config_root_for(platform)
        .unwrap_or_else(|| home.join(".config"));

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
            notes: vec!["Linux follows the same ~/.codex layout as macOS.".into()],
        },
        IntegrationSpec {
            integration: IntegrationKind::ClaudeCode,
            platform,
            config_target: IntegrationPathTarget::file(
                home.join(".claude").join("mcp").join("mentisdb.json"),
                "User-level Claude Code MCP entry",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                home.join(".claude"),
                "Claude Code home directory",
            )],
            companion_targets: vec![IntegrationPathTarget::file(
                home.join(".claude").join("settings.json"),
                "General Claude Code settings",
                IntegrationFileFormat::Json,
            )],
            notes: vec!["Linux path mapping follows ~/.claude/mcp/mentisdb.json.".into()],
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
                "Gemini Linux support is path-mapped from the macOS layout; host validation should happen when the writer lands."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::OpenCode,
            platform,
            config_target: IntegrationPathTarget::file(
                config_root.join("opencode").join("opencode.json"),
                "Primary OpenCode settings",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                config_root.join("opencode"),
                "OpenCode config directory",
            )],
            companion_targets: vec![],
            notes: vec!["OpenCode uses an XDG-style config directory on Linux.".into()],
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
            notes: vec!["Qwen Linux support follows ~/.qwen/settings.json.".into()],
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
                config_root.join("Code").join("User").join("mcp.json"),
                "VS Code MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                config_root.join("Code").join("User"),
                "VS Code user settings directory",
            )],
            companion_targets: vec![IntegrationPathTarget::file(
                config_root.join("Code").join("User").join("settings.json"),
                "Existing VS Code user settings",
                IntegrationFileFormat::Json,
            )],
            notes: vec![
                "Use the Code/User directory as the installation signal even if mcp.json has not been created yet."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::ClaudeDesktop,
            platform,
            config_target: IntegrationPathTarget::file(
                config_root
                    .join("Claude")
                    .join("claude_desktop_config.json"),
                "Claude Desktop MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                config_root.join("Claude"),
                "Claude Desktop config directory",
            )],
            companion_targets: vec![],
            notes: vec![
                "Linux Claude Desktop support is path-mapped to ~/.config/Claude/claude_desktop_config.json."
                    .into(),
            ],
        },
    ]
}

fn user_root(env: &PathEnvironment, platform: HostPlatform) -> PathBuf {
    env.home_dir_for(platform)
        .or_else(|| env.current_dir.clone())
        .unwrap_or_else(|| PathBuf::from("."))
}
