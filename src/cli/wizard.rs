use crate::integrations::detect::{detect_integrations_with_environment, DetectionStatus};
use crate::integrations::plan::{
    build_detected_setup_catalog, build_setup_plan_for_integration, SetupCatalogPlan,
};
use crate::integrations::IntegrationKind;
use crate::paths::{HostPlatform, PathEnvironment};
use std::io::{self, BufRead, Write};

use super::args::{default_url, parse_integration};
use super::prompt::{boxed_apply_summary, boxed_skip_notice, boxed_text_prompt, boxed_yn_prompt};
use super::setup::ensure_prerequisites;
use super::{render_setup_plan, WizardCommand};

pub(super) fn run_wizard(
    command: &WizardCommand,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
) -> io::Result<()> {
    let env = PathEnvironment::capture();
    let platform = HostPlatform::current();
    let report = detect_integrations_with_environment(platform, env.clone());
    let catalog = build_detected_setup_catalog(report);

    writeln!(out, "MentisDB setup wizard")?;
    writeln!(out)?;
    render_catalog_summary(&catalog, out)?;
    writeln!(out)?;

    let selected = if command.assume_yes {
        default_selections(&catalog)
    } else {
        let prompt = format!(
            "Select integrations [default: {}], 'all', or 'none'",
            default_selection_label(&catalog)
        );
        let entered = boxed_text_prompt(out, &prompt, input)?;
        resolve_selections(&catalog, &entered)?
    };

    if selected.is_empty() {
        writeln!(out, "\nNothing selected.")?;
        return Ok(());
    }

    let url_override = if let Some(url) = &command.url {
        Some(url.clone())
    } else {
        let entered = boxed_text_prompt(
            out,
            "Override the default MentisDB URL for all selected integrations?\n(Leave blank to use per-integration defaults)",
            input,
        )?;
        (!entered.trim().is_empty()).then_some(entered.trim().to_string())
    };

    let mut planned = Vec::new();
    for integration in selected {
        let url = url_override
            .clone()
            .unwrap_or_else(|| default_url(integration).to_string());
        let Some(plan) = build_setup_plan_for_integration(integration, url, platform, &env) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "unsupported integration target",
            ));
        };
        planned.push(plan);
    }

    let mut final_plans = Vec::new();
    for plan in planned {
        if plan.detection_status == DetectionStatus::Configured {
            if command.assume_yes {
                boxed_skip_notice(out, plan.integration.display_name())?;
                continue;
            }

            let question = format!(
                "{} already has a mentisdb integration.\nOverwrite or keep the existing config?",
                plan.integration.display_name()
            );
            let decision = boxed_yn_prompt(out, &question, false, input)?;
            if !decision.eq_ignore_ascii_case("y") && !decision.eq_ignore_ascii_case("yes") {
                continue;
            }
        }
        write!(out, "{}", render_setup_plan(&plan))?;
        final_plans.push(plan);
    }

    if final_plans.is_empty() {
        return Ok(());
    }

    let default_yes = !command.assume_yes;
    let apply_items: Vec<(String, String)> = final_plans
        .iter()
        .map(|plan| {
            (
                plan.integration.display_name().to_owned(),
                plan.spec.config_target.path.display().to_string(),
            )
        })
        .collect();

    if !command.assume_yes {
        let response = boxed_apply_summary(out, &apply_items, default_yes, input)?;
        if response.eq_ignore_ascii_case("n") || response.eq_ignore_ascii_case("no") {
            writeln!(out, "\nCancelled.")?;
            return Ok(());
        }
    }

    writeln!(out)?;
    for plan in final_plans {
        ensure_prerequisites(plan.integration, out)?;
        let result = crate::integrations::apply::apply_setup_with_environment(
            plan.integration,
            plan.url.clone(),
            platform,
            &env,
        )?;
        writeln!(
            out,
            "{} -> {} ({})",
            plan.integration.display_name(),
            result.path.display(),
            if result.changed {
                "updated"
            } else {
                "unchanged"
            }
        )?;
    }

    Ok(())
}

fn render_catalog_summary(catalog: &SetupCatalogPlan, out: &mut dyn Write) -> io::Result<()> {
    writeln!(out, "Detected integrations:")?;
    for (index, plan) in catalog.integrations.iter().enumerate() {
        writeln!(
            out,
            "  {}. {:<20} {:<18} {}",
            index + 1,
            plan.integration.display_name(),
            plan.detection_status.as_str(),
            plan.spec.config_target.path.display()
        )?;
    }
    Ok(())
}

fn default_selections(catalog: &SetupCatalogPlan) -> Vec<IntegrationKind> {
    catalog
        .integrations
        .iter()
        .filter(|plan| plan.detection_status == DetectionStatus::InstalledOrUsed)
        .map(|plan| plan.integration)
        .collect()
}

fn default_selection_label(catalog: &SetupCatalogPlan) -> String {
    let defaults = default_selections(catalog);
    if defaults.is_empty() {
        "none".to_string()
    } else {
        defaults
            .iter()
            .map(|integration| integration.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn resolve_selections(catalog: &SetupCatalogPlan, raw: &str) -> io::Result<Vec<IntegrationKind>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(default_selections(catalog));
    }
    if trimmed.eq_ignore_ascii_case("none") {
        return Ok(Vec::new());
    }
    if trimmed.eq_ignore_ascii_case("all") {
        return Ok(catalog
            .integrations
            .iter()
            .map(|plan| plan.integration)
            .collect());
    }

    let mut selections = Vec::new();
    for part in trimmed.split(',') {
        let Some(selection) = parse_selection(part.trim()) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid selection '{}'", part.trim()),
            ));
        };
        if !selections.contains(&selection) {
            selections.push(selection);
        }
    }
    Ok(selections)
}

fn parse_selection(value: &str) -> Option<IntegrationKind> {
    if let Ok(index) = value.trim().parse::<usize>() {
        if index == 0 {
            return None;
        }
        return IntegrationKind::ALL.get(index - 1).copied();
    }

    parse_integration(value)
}
