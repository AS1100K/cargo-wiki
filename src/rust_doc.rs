use anyhow::Result;
use std::process::Command;
use crate::Configuration;

pub fn generate_rust_doc(configuration: &Configuration) -> Result<()> {
    let mut command = Command::new("cargo")
        .arg("+nightly")
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
