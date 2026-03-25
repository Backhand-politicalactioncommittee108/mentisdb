//! MentisDB command-line interface for setup and onboarding workflows.

use std::process::ExitCode;

fn main() -> ExitCode {
    mentisdb::cli::run()
}
