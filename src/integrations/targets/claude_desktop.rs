use crate::integrations::files::{JsonPatch, ManagedFile};
use crate::integrations::plan::SetupPlan;
use crate::integrations::state::{IntegrationApplyPlan, IntegrationWriterSettings};
use serde_json::json;

pub(super) fn build(
    plan: &SetupPlan,
    settings: &IntegrationWriterSettings,
) -> IntegrationApplyPlan {
    let url = settings.url_for(plan.integration);
    let mut patch = JsonPatch::new()
        .set_path(
            ["mcpServers", settings.server_name(), "command"],
            json!(settings.bridge_command_for(plan.platform)),
        )
        .set_path(["mcpServers", settings.server_name(), "args"], json!([url]));

    if url.starts_with("https://") {
        patch = patch.set_path(
            [
                "mcpServers",
                settings.server_name(),
                "env",
                "NODE_TLS_REJECT_UNAUTHORIZED",
            ],
            json!("0"),
        );
    }

    IntegrationApplyPlan::new(plan.integration, plan.platform).with_file(ManagedFile::json(
        plan.spec.config_target.path.clone(),
        patch,
    ))
}
