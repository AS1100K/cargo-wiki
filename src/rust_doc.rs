use crate::Configuration;
use anyhow::Result;
use std::process::Command;

pub fn generate_rust_doc(configuration: &Configuration) -> Result<()> {
    let mut cargo = Command::new("cargo");
    let mut command = cargo
        .arg("rustdoc")
        .arg("-Zunstable-options")
        .arg("--output-format")
        .arg("json");

    // package
    if !configuration.package.is_empty() {
        command = command.args(vec!["-p", &configuration.package]);
    }

    // features
    if !configuration.features.is_empty() {
        command = command.args(vec!["--features", &configuration.features]);
    }

    // all-features
    if configuration.all_features {
        command = command.arg("--all-features");
    }

    // no-default-features
    if configuration.all_features {
        command = command.arg("--no-default-features");
    }

    let mut child = command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdout) = child.stdout.take() {
        std::io::copy(&mut stdout, &mut std::io::stdout())?;
    }

    if let Some(mut stderr) = child.stderr.take() {
        std::io::copy(&mut stderr, &mut std::io::stderr())?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Command execution failed"));
    }

    Ok(())
}
