use crate::integrations::files::{JsonPatch, ManagedFile};
use crate::integrations::plan::SetupPlan;
use crate::integrations::state::{IntegrationApplyPlan, IntegrationWriterSettings};
use serde_json::json;

pub(super) fn build(
    plan: &SetupPlan,
    settings: &IntegrationWriterSettings,
) -> IntegrationApplyPlan {
    let patch = JsonPatch::new()
        .ensure_path(["$version"], json!(3))
        .set_path(
            ["mcpServers", settings.server_name(), "httpUrl"],
            json!(settings.url_for(plan.integration)),
        );

    IntegrationApplyPlan::new(plan.integration, plan.platform).with_file(ManagedFile::json(
        plan.spec.config_target.path.clone(),
        patch,
    ))
}
