use crate::Configuration;
use anyhow::Result;
use std::process::Command;

pub fn generate_rust_doc(configuration: &Configuration) -> Result<()> {
    // TODO: Modify the command based on the configuration
    let mut command = Command::new("cargo")
        .arg("rustdoc")
        .arg("-Zunstable-options")
        .arg("--output-format")
        .arg("json")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdout) = command.stdout.take() {
        std::io::copy(&mut stdout, &mut std::io::stdout())?;
    }

    if let Some(mut stderr) = command.stderr.take() {
        std::io::copy(&mut stderr, &mut std::io::stderr())?;
    }

    let status = command.wait()?;
    if !status.success() {
        return Err(anyhow::anyhow!("Command execution failed"));
    }

    Ok(())
}
