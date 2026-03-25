use crate::integrations::files::{JsonPatch, ManagedFile};
use crate::integrations::plan::SetupPlan;
use crate::integrations::state::{IntegrationApplyPlan, IntegrationWriterSettings};
use serde_json::json;

pub(super) fn build(
    plan: &SetupPlan,
    settings: &IntegrationWriterSettings,
) -> IntegrationApplyPlan {
    let patch = JsonPatch::new()
        .set_path(
            ["mcpServers", settings.server_name(), "type"],
            json!("http"),
        )
        .set_path(
            ["mcpServers", settings.server_name(), "url"],
            json!(settings.url_for(plan.integration)),
        )
        .set_path(["mcpServers", settings.server_name(), "headers"], json!({}))
        .set_path(
            ["mcpServers", settings.server_name(), "tools"],
            json!(["*"]),
        );

    IntegrationApplyPlan::new(plan.integration, plan.platform).with_file(ManagedFile::json(
        plan.spec.config_target.path.clone(),
        patch,
    ))
}
