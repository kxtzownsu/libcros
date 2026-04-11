use std::process::{Command, ExitStatus, Stdio};

/// Run a shell command.
/// Returns output text or error text.
pub fn execute_cmd_stdio(command: &str, live_output: bool) -> String {
  if live_output {
    let status = Command::new("bash")
      .arg("-c")
      .arg(command)
      .stdin(Stdio::inherit())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status();

    match status {
      Ok(s) if s.success() => "".to_string(),
      Ok(s) => format!("process exited with status: {}", s),
      Err(e) => format!("error executing command: {}", e),
    }
  } else {
    match Command::new("bash").arg("-c").arg(command).output() {
      Ok(output) => {
        if output.status.success() {
          String::from_utf8_lossy(&output.stdout).to_string()
        } else {
          String::from_utf8_lossy(&output.stderr).to_string()
        }
      }
      Err(e) => format!("error executing command: {}", e),
    }
  }
}

/// Run a shell command and return exit code.
/// Returns -1 on failure.
pub fn execute_cmd_rc(command: &str, live_output: bool) -> i32 {
  if live_output {
    match Command::new("bash")
      .arg("-c")
      .arg(command)
      .stdin(Stdio::inherit())
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .status()
    {
      Ok(s) => s.code().unwrap_or(-1),
      Err(_) => -1,
    }
  } else {
    match Command::new("bash").arg("-c").arg(command).output() {
      Ok(output) => output.status.code().unwrap_or(-1),
      Err(_) => -1,
    }
  }
}

/// Spawn /bin/bash and wait for exit.
pub fn spawn_bash_shell() -> ExitStatus {
  let err = Command::new("/bin/bash")
    .spawn()
    .expect("failed to spawn bash")
    .wait()
    .expect("failed to wait on bash");

  return err;
}
