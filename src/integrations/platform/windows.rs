use crate::integrations::{
    IntegrationFileFormat, IntegrationKind, IntegrationPathTarget, IntegrationSpec,
};
use crate::paths::{HostPlatform, PathEnvironment};
use std::path::PathBuf;

pub(super) fn specs(env: &PathEnvironment) -> Vec<IntegrationSpec> {
    let platform = HostPlatform::Windows;
    let home = user_root(env, platform);
    let app_data = env
        .config_root_for(platform)
        .unwrap_or_else(|| home.join("AppData").join("Roaming"));

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
            notes: vec!["Windows Codex support maps to %USERPROFILE%\\.codex\\config.toml.".into()],
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
            notes: vec!["Windows Claude Code support maps to %USERPROFILE%\\.claude\\mcp\\mentisdb.json.".into()],
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
            notes: vec!["Windows Gemini path mapping mirrors the user-home layout used on macOS.".into()],
        },
        IntegrationSpec {
            integration: IntegrationKind::OpenCode,
            platform,
            config_target: IntegrationPathTarget::file(
                app_data.join("opencode").join("opencode.json"),
                "Primary OpenCode settings",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                app_data.join("opencode"),
                "OpenCode config directory",
            )],
            companion_targets: vec![],
            notes: vec!["Windows OpenCode support maps to %APPDATA%\\opencode\\opencode.json.".into()],
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
            notes: vec!["Windows Qwen support maps to %USERPROFILE%\\.qwen\\settings.json.".into()],
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
                "Windows GitHub Copilot CLI support maps to %USERPROFILE%\\.copilot\\mcp-config.json."
                    .into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::VsCodeCopilot,
            platform,
            config_target: IntegrationPathTarget::file(
                app_data.join("Code").join("User").join("mcp.json"),
                "VS Code MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                app_data.join("Code").join("User"),
                "VS Code user settings directory",
            )],
            companion_targets: vec![IntegrationPathTarget::file(
                app_data.join("Code").join("User").join("settings.json"),
                "Existing VS Code user settings",
                IntegrationFileFormat::Json,
            )],
            notes: vec![
                "Treat %APPDATA%\\Code\\User as the installation signal even when mcp.json is absent.".into(),
            ],
        },
        IntegrationSpec {
            integration: IntegrationKind::ClaudeDesktop,
            platform,
            config_target: IntegrationPathTarget::file(
                app_data.join("Claude").join("claude_desktop_config.json"),
                "Claude Desktop MCP configuration",
                IntegrationFileFormat::Json,
            ),
            detection_probes: vec![IntegrationPathTarget::directory(
                app_data.join("Claude"),
                "Claude Desktop config directory",
            )],
            companion_targets: vec![],
            notes: vec![
                "Windows Claude Desktop support maps to %APPDATA%\\Claude\\claude_desktop_config.json."
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
