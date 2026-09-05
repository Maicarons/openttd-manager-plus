//! Game process runner and lifecycle management.
//!
//! Provides the [`GameRunner`] for starting OpenTTD processes
//! and the [`GameProcess`] handle for controlling a running game
//! instance with platform-specific process group management.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

use crate::{Error, Result};

/// Configuration for launching an OpenTTD game process.
#[derive(Debug, Clone)]
pub struct GameRunnerConfig {
    /// Path to the OpenTTD executable.
    pub executable: PathBuf,
    /// Working directory for the game process.
    pub working_dir: PathBuf,
    /// Command-line arguments passed to the executable.
    pub args: Vec<String>,
    /// Environment variables set for the game process.
    pub env_vars: HashMap<String, String>,
}

impl Default for GameRunnerConfig {
    fn default() -> Self {
        Self {
            executable: PathBuf::from("openttd"),
            working_dir: PathBuf::from("."),
            args: Vec::new(),
            env_vars: HashMap::new(),
        }
    }
}

/// Launches and manages OpenTTD game processes.
///
/// Wraps a [`GameRunnerConfig`] and provides the [`start`](GameRunner::start)
/// method to spawn the game process.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use std::path::PathBuf;
/// use otmp_core_launcher::{GameRunner, GameRunnerConfig};
///
/// let config = GameRunnerConfig {
///     executable: PathBuf::from("openttd"),
///     working_dir: PathBuf::from("."),
///     args: vec!["-D".into(), "./data".into()],
///     env_vars: HashMap::new(),
/// };
/// let runner = GameRunner::new(config);
/// // runner.start() would spawn the process
/// ```
#[derive(Debug)]
pub struct GameRunner {
    config: GameRunnerConfig,
}

impl GameRunner {
    /// Creates a new `GameRunner` with the given configuration.
    pub fn new(config: GameRunnerConfig) -> Self {
        Self { config }
    }

    /// Returns a reference to the runner's configuration.
    pub fn config(&self) -> &GameRunnerConfig {
        &self.config
    }

    /// Starts the game process.
    ///
    /// Validates that the executable exists before attempting to spawn.
    /// On Windows, the process is created with the `CREATE_NO_WINDOW` flag.
    /// On Unix, the process is placed in a new session (via `setsid`) so
    /// that the entire process group can be terminated later.
    ///
    /// # Errors
    ///
    /// Returns [`Error::GameNotFound`] if the executable does not exist.
    /// Returns [`Error::LaunchFailed`] if the process could not be spawned.
    pub fn start(&self) -> Result<GameProcess> {
        if !self.config.executable.exists() {
            return Err(Error::GameNotFound(format!(
                "Executable not found: {}",
                self.config.executable.display()
            )));
        }

        let mut cmd = std::process::Command::new(&self.config.executable);
        cmd.current_dir(&self.config.working_dir);
        cmd.args(&self.config.args);
        cmd.envs(&self.config.env_vars);

        // Platform-specific process creation flags
        #[cfg(windows)]
        {
            // CREATE_NO_WINDOW = 0x08000000: prevents the process from
            // creating a console window (useful for background launchers).
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        #[cfg(unix)]
        {
            // Create a new session so the child and its descendants form
            // an independent process group that can be killed as a unit.
            unsafe {
                cmd.pre_exec(|| {
                    libc::setsid();
                    Ok(())
                });
            }
        }

        // Inherit stdio so the game's output is visible to the user.
        cmd.stdin(std::process::Stdio::inherit());
        cmd.stdout(std::process::Stdio::inherit());
        cmd.stderr(std::process::Stdio::inherit());

        log::info!(
            "Launching game: {} {}",
            self.config.executable.display(),
            self.config.args.join(" ")
        );

        let child = cmd.spawn().map_err(|e| {
            log::error!("Failed to launch game: {}", e);
            Error::LaunchFailed(e.to_string())
        })?;

        log::info!("Game started (PID: {})", child.id());

        Ok(GameProcess { child })
    }
}

/// Handle to a running OpenTTD game process.
///
/// Provides methods to query the process state, wait for it to exit,
/// or terminate it. On Unix, termination targets the entire process
/// group to ensure child processes are also cleaned up.
///
/// The process handle is associated with a specific platform.
/// Dropping the handle does **not** kill the process — it becomes
/// detached from supervision.
#[derive(Debug)]
pub struct GameProcess {
    child: std::process::Child,
}

impl GameProcess {
    /// Returns the process ID of the game.
    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    /// Checks if the game process is still running.
    ///
    /// Returns `Ok(true)` if the process is alive, `Ok(false)` if it
    /// has exited. Returns an error if the process state could not be
    /// queried.
    pub fn is_running(&mut self) -> Result<bool> {
        match self.child.try_wait() {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(e) => Err(Error::from(e)),
        }
    }

    /// Blocks until the game process exits.
    ///
    /// Returns the exit status of the process.
    pub fn wait(&mut self) -> Result<std::process::ExitStatus> {
        log::info!("Waiting for game process (PID: {}) to exit...", self.child.id());
        let status = self.child.wait()?;
        log::info!("Game process (PID: {}) exited with: {}", self.child.id(), status);
        Ok(status)
    }

    /// Attempts to wait for the process to exit with a timeout.
    ///
    /// Polls the process state at 50 ms intervals. Returns:
    /// - `Ok(Some(status))` if the process exited within the timeout.
    /// - `Ok(None)` if the timeout elapsed and the process is still running.
    /// - `Err(e)` if an I/O error occurs.
    pub fn wait_timeout(&mut self, timeout: Duration) -> Result<Option<std::process::ExitStatus>> {
        let start = std::time::Instant::now();
        let poll_interval = Duration::from_millis(50);

        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    log::info!(
                        "Game process (PID: {}) exited with: {}",
                        self.child.id(),
                        status
                    );
                    return Ok(Some(status));
                }
                Ok(None) => {
                    if start.elapsed() >= timeout {
                        return Ok(None);
                    }
                    std::thread::sleep(poll_interval);
                }
                Err(e) => {
                    log::error!("Failed to query game process (PID: {}): {}", self.child.id(), e);
                    return Err(Error::from(e));
                }
            }
        }
    }

    /// Forces the game process to terminate.
    ///
    /// On Unix, sends `SIGTERM` to the entire process group.
    /// On Windows, terminates the process tree via `taskkill /T`.
    ///
    /// After killing, the process should be reaped by calling
    /// [`wait`](GameProcess::wait) or [`wait_timeout`](GameProcess::wait_timeout).
    pub fn kill(&mut self) -> Result<()> {
        let pid = self.child.id();
        log::info!("Killing game process (PID: {})", pid);

        #[cfg(unix)]
        {
            // Send SIGTERM to the entire process group (negative PID).
            // The child was started with setsid(), so its PGID equals its PID.
            let result = unsafe { libc::kill(-(pid as i32), libc::SIGTERM) };
            if result != 0 {
                let err = std::io::Error::last_os_error();
                log::warn!(
                    "kill(-{}, SIGTERM) failed ({}), falling back to kill()",
                    pid,
                    err
                );
                self.child.kill()?;
            }
        }

        #[cfg(windows)]
        {
            // On Windows, use taskkill to terminate the process tree.
            let output = std::process::Command::new("taskkill")
                .args(["/F", "/T", "/PID", &pid.to_string()])
                .output()
                .map_err(|e| {
                    log::warn!("taskkill failed, falling back to kill(): {}", e);
                    // Fall through to the unconditional kill below
                });

            match output {
                Ok(o) if o.status.success() => {
                    log::info!("taskkill /T succeeded for PID {}", pid);
                }
                _ => {
                    self.child.kill()?;
                }
            }
        }

        log::info!("Game process (PID: {}) terminated", pid);
        Ok(())
    }
}

impl Drop for GameProcess {
    /// Detaches from the process without killing it.
    ///
    /// This is intentional: dropping the handle releases supervision
    /// but does not terminate the game. Use [`kill`](GameProcess::kill)
    /// explicitly if termination is required.
    fn drop(&mut self) {
        // The child is detached — we do not kill it on drop.
        let _ = self.child.id();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_config_default() {
        let config = GameRunnerConfig::default();
        assert_eq!(config.executable, PathBuf::from("openttd"));
        assert_eq!(config.working_dir, PathBuf::from("."));
        assert!(config.args.is_empty());
        assert!(config.env_vars.is_empty());
    }

    #[test]
    fn test_runner_new() {
        let config = GameRunnerConfig::default();
        let runner = GameRunner::new(config.clone());
        assert_eq!(runner.config().executable, config.executable);
    }

    #[test]
    fn test_runner_game_not_found() {
        let config = GameRunnerConfig {
            executable: PathBuf::from("/nonexistent/openttd"),
            ..Default::default()
        };
        let runner = GameRunner::new(config);
        let result = runner.start();
        assert!(result.is_err());
        match result {
            Err(Error::GameNotFound(_)) => {} // expected
            _ => panic!("Expected GameNotFound error"),
        }
    }
}