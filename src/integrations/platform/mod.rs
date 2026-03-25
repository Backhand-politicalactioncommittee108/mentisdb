//! Platform-specific path catalogs for host integrations.

mod linux;
mod macos;
mod windows;

use crate::integrations::IntegrationSpec;
use crate::paths::{HostPlatform, PathEnvironment};

/// Build the integration catalog for the requested platform.
pub fn specs_for(platform: HostPlatform, env: &PathEnvironment) -> Vec<IntegrationSpec> {
    match platform {
        HostPlatform::Macos => macos::specs(env),
        HostPlatform::Linux | HostPlatform::Other => linux::specs(platform, env),
        HostPlatform::Windows => windows::specs(env),
    }
}
