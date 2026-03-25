//! Shared CLI helpers for the `mentisdb` binary.
//!
//! The binary entrypoint stays intentionally thin and delegates parsing plus
//! wizard/setup behavior to this module so the logic is directly testable.

mod args;
mod setup;
mod wizard;

pub use args::{parse_args, CliCommand, SetupCommand, WizardCommand};
pub use setup::render_setup_plan;

use std::ffi::OsString;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process::ExitCode;

/// Run the `mentisdb` CLI using process arguments and stdio.
pub fn run() -> ExitCode {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stderr = io::stderr();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    let mut errors = stderr.lock();

    run_with_io(std::env::args_os(), &mut input, &mut output, &mut errors)
}

/// Run the `mentisdb` CLI with caller-provided streams.
pub fn run_with_io<I, T>(
    args: I,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> ExitCode
where
    I: IntoIterator<Item = T>,
    T: Into<OsString>,
{
    match parse_args(args) {
        Ok(CliCommand::Help) => {
            if io::stdin().is_terminal() && io::stdout().is_terminal() && !wizard_state_seen() {
                match wizard::run_first_run_wizard(input, out) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(error) => {
                        let _ = writeln!(err, "wizard failed: {error}");
                        ExitCode::from(1)
                    }
                }
            } else {
                let _ = write!(out, "{}", args::help_text());
                ExitCode::SUCCESS
            }
        }
        Ok(CliCommand::Setup(command)) => match setup::run_setup(&command, out) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                let _ = writeln!(err, "setup failed: {error}");
                ExitCode::from(1)
            }
        },
        Ok(CliCommand::Wizard(command)) => match wizard::run_wizard(&command, input, out) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                let _ = writeln!(err, "wizard failed: {error}");
                ExitCode::from(1)
            }
        },
        Err(message) => {
            let _ = writeln!(err, "{message}");
            let _ = writeln!(err);
            let _ = write!(err, "{}", args::help_text());
            ExitCode::from(2)
        }
    }
}

fn wizard_state_seen() -> bool {
    crate::paths::default_mentisdb_dir()
        .join("cli-wizard-state.json")
        .exists()
}
