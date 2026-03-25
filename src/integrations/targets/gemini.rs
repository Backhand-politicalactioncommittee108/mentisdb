use crate::integrations::files::{JsonPatch, ManagedFile};
use crate::integrations::plan::SetupPlan;
use crate::integrations::state::{IntegrationApplyPlan, IntegrationWriterSettings};
use serde_json::json;

pub(super) fn build(
    plan: &SetupPlan,
    settings: &IntegrationWriterSettings,
) -> IntegrationApplyPlan {
    let url = settings.url_for(plan.integration);
    let patch = JsonPatch::new()
        .set_path(
            ["mcpServers", settings.server_name(), "type"],
            json!("http"),
        )
        .set_path(["mcpServers", settings.server_name(), "url"], json!(url))
        .set_path(
            ["mcpServers", settings.server_name(), "httpUrl"],
            json!(url),
        );

    IntegrationApplyPlan::new(plan.integration, plan.platform).with_file(ManagedFile::json(
        plan.spec.config_target.path.clone(),
        patch,
    ))
}
