//! Shared state for concrete integration apply plans.

use crate::integrations::files::ManagedFile;
use crate::integrations::IntegrationKind;
use crate::paths::HostPlatform;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

/// Shared writer settings for MentisDB host integrations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IntegrationWriterSettings {
    /// Stable MCP server name written into client config files.
    server_name: String,
    /// Default local streamable HTTP MCP URL used by most clients.
    default_http_url: String,
    /// Default HTTPS MCP URL used by bridge-based clients.
    default_https_url: String,
    /// Optional explicit `mcp-remote` bridge command for Claude Desktop.
    claude_desktop_bridge_command: Option<String>,
}

impl Default for IntegrationWriterSettings {
    fn default() -> Self {
        Self {
            server_name: "mentisdb".to_owned(),
            default_http_url: "http://127.0.0.1:9471".to_owned(),
            default_https_url: "https://my.mentisdb.com:9473".to_owned(),
            claude_desktop_bridge_command: None,
        }
    }
}

impl IntegrationWriterSettings {
    pub(crate) fn with_url_for(
        &self,
        integration: IntegrationKind,
        url: impl Into<String>,
    ) -> Self {
        let mut next = self.clone();
        let url = url.into();
        match integration {
            IntegrationKind::ClaudeDesktop => next.default_https_url = url,
            _ => next.default_http_url = url,
        }
        next
    }

    pub(crate) fn server_name(&self) -> &str {
        &self.server_name
    }

    pub(crate) fn url_for(&self, integration: IntegrationKind) -> &str {
        match integration {
            IntegrationKind::ClaudeDesktop => &self.default_https_url,
            _ => &self.default_http_url,
        }
    }

    pub(crate) fn bridge_command_for(&self, platform: HostPlatform) -> String {
        self.claude_desktop_bridge_command
            .clone()
            .unwrap_or_else(|| detect_mcp_remote_command(platform))
    }
}

/// Fully expanded file-write plan for one integration target.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IntegrationApplyPlan {
    /// Integration that will be updated.
    pub(crate) integration: IntegrationKind,
    /// Platform-specific path mapping used for the plan.
    pub(crate) platform: HostPlatform,
    /// Managed files to merge or create.
    pub(crate) files: Vec<ManagedFile>,
}

impl IntegrationApplyPlan {
    pub(crate) fn new(integration: IntegrationKind, platform: HostPlatform) -> Self {
        Self {
            integration,
            platform,
            files: Vec::new(),
        }
    }

    pub(crate) fn with_file(mut self, file: ManagedFile) -> Self {
        self.files.push(file);
        self
    }
}

fn detect_mcp_remote_command(platform: HostPlatform) -> String {
    let binary_name = match platform {
        HostPlatform::Windows => "mcp-remote.cmd",
        _ => "mcp-remote",
    };

    for entry in split_path_entries(env::var_os("PATH")) {
        let candidate = entry.join(binary_name);
        if is_executable_file(&candidate) {
            return candidate.display().to_string();
        }
    }

    if matches!(platform, HostPlatform::Macos) {
        for candidate in [
            PathBuf::from("/opt/homebrew/bin/mcp-remote"),
            PathBuf::from("/usr/local/bin/mcp-remote"),
        ] {
            if is_executable_file(&candidate) {
                return candidate.display().to_string();
            }
        }
    }

    binary_name.to_owned()
}

fn split_path_entries(path: Option<OsString>) -> Vec<PathBuf> {
    path.map(|value| env::split_paths(&value).collect())
        .unwrap_or_default()
}

fn is_executable_file(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}
