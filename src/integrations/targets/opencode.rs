use crate::integrations::files::{JsonPatch, ManagedFile};
use crate::integrations::plan::SetupPlan;
use crate::integrations::state::{IntegrationApplyPlan, IntegrationWriterSettings};
use serde_json::json;

pub(super) fn build(
    plan: &SetupPlan,
    settings: &IntegrationWriterSettings,
) -> IntegrationApplyPlan {
    let patch = JsonPatch::new()
        .set_path(["mcp", settings.server_name(), "type"], json!("remote"))
        .set_path(
            ["mcp", settings.server_name(), "url"],
            json!(settings.url_for(plan.integration)),
        )
        .set_path(["mcp", settings.server_name(), "enabled"], json!(true));

    IntegrationApplyPlan::new(plan.integration, plan.platform).with_file(ManagedFile::json(
        plan.spec.config_target.path.clone(),
        patch,
    ))
}
