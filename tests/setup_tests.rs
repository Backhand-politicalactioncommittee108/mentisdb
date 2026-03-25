use mentisdb::cli::{parse_args, render_setup_plan, CliCommand, SetupCommand};
use mentisdb::integrations::plan::build_setup_plan_for_integration;
use mentisdb::integrations::IntegrationKind;
use mentisdb::paths::{HostPlatform, PathEnvironment};

#[test]
fn parse_setup_command_accepts_supported_agent_and_url_override() {
    let parsed = parse_args([
        "mentisdb",
        "setup",
        "codex",
        "--url",
        "http://127.0.0.1:9999",
    ])
    .unwrap();

    assert_eq!(
        parsed,
        CliCommand::Setup(SetupCommand {
            integrations: vec![IntegrationKind::Codex],
            url: Some("http://127.0.0.1:9999".to_string()),
            dry_run: false,
        })
    );
}

#[test]
fn parse_setup_help_returns_help_command() {
    let parsed = parse_args(["mentisdb", "setup", "--help"]).unwrap();
    assert_eq!(parsed, CliCommand::Help);
}

#[test]
fn parse_setup_command_keeps_per_integration_defaults_when_url_is_omitted() {
    let parsed = parse_args(["mentisdb", "setup", "claude-desktop"]).unwrap();

    assert_eq!(
        parsed,
        CliCommand::Setup(SetupCommand {
            integrations: vec![IntegrationKind::ClaudeDesktop],
            url: None,
            dry_run: false,
        })
    );
}

#[test]
fn macos_vscode_copilot_plan_uses_application_support_path() {
    let env = PathEnvironment {
        home_dir: Some("/Users/tester".into()),
        ..PathEnvironment::default()
    };
    let plan = build_setup_plan_for_integration(
        IntegrationKind::VsCodeCopilot,
        "http://127.0.0.1:9471",
        HostPlatform::Macos,
        &env,
    )
    .unwrap();

    assert_eq!(
        plan.spec.config_target.path,
        std::path::PathBuf::from("/Users/tester/Library/Application Support/Code/User/mcp.json")
    );
    assert!(plan
        .snippet
        .as_deref()
        .unwrap()
        .contains("\"type\": \"http\""));
}

#[test]
fn rendered_setup_plan_includes_status_and_action() {
    let env = PathEnvironment {
        home_dir: Some("/Users/tester".into()),
        ..PathEnvironment::default()
    };
    let plan = build_setup_plan_for_integration(
        IntegrationKind::Codex,
        "http://127.0.0.1:9471",
        HostPlatform::Macos,
        &env,
    )
    .unwrap();
    let rendered = render_setup_plan(&plan);

    assert!(rendered.contains("Status:"));
    assert!(rendered.contains("Action:"));
    assert!(rendered.contains("codex mcp add mentisdb --url http://127.0.0.1:9471"));
}

#[test]
fn help_text_lists_all_supported_agents_and_commands() {
    let help = mentisdb::cli::parse_args(["mentisdb", "--help"]);
    assert!(help.is_ok());

    let text = {
        use mentisdb::cli::run_with_io;
        use std::io::Cursor;
        let mut input = Cursor::new(Vec::<u8>::new());
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let _ = run_with_io(["mentisdb", "--help"], &mut input, &mut output, &mut errors);
        String::from_utf8(output).unwrap()
    };

    for agent in [
        "codex",
        "claude-code",
        "claude-desktop",
        "gemini",
        "opencode",
        "qwen",
        "copilot",
        "vscode-copilot",
    ] {
        assert!(text.contains(agent), "missing {agent} in help text");
    }
    assert!(text.contains("mentisdb setup <agent|all>"));
    assert!(text.contains("mentisdb wizard"));
}
