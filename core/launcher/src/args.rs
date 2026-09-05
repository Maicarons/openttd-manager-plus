//! OpenTTD command-line argument builder.
//!
//! Provides a builder pattern for constructing OpenTTD command-line
//! arguments following the standard OpenTTD CLI conventions.

use std::path::PathBuf;

/// Builder for OpenTTD command-line arguments.
///
/// Constructs the argument vector for launching OpenTTD
/// following the standard OpenTTD CLI conventions.
///
/// # Example
///
/// ```rust
/// use otmp_core_launcher::ArgsBuilder;
/// use std::path::PathBuf;
///
/// let args = ArgsBuilder::new()
///     .data_dir(PathBuf::from("/home/user/openttd-data"))
///     .resolution(1920, 1080)
///     .fullscreen(true)
///     .build();
///
/// assert_eq!(args, vec![
///     "-D", "/home/user/openttd-data",
///     "-r", "1920x1080",
///     "-f",
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct ArgsBuilder {
    data_dir: Option<PathBuf>,
    config_file: Option<PathBuf>,
    save_game: Option<PathBuf>,
    connect_host: Option<String>,
    connect_port: Option<u16>,
    connect_password: Option<String>,
    debug: bool,
    resolution: Option<(u32, u32)>,
    fullscreen: bool,
    custom_args: Vec<String>,
}

impl Default for ArgsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ArgsBuilder {
    /// Creates a new `ArgsBuilder` with all options set to their defaults.
    ///
    /// No arguments are set by default. Each option must be explicitly
    /// configured before calling [`build`](ArgsBuilder::build).
    pub fn new() -> Self {
        Self {
            data_dir: None,
            config_file: None,
            save_game: None,
            connect_host: None,
            connect_port: None,
            connect_password: None,
            debug: false,
            resolution: None,
            fullscreen: false,
            custom_args: Vec::new(),
        }
    }

    /// Sets the OpenTTD data directory.
    ///
    /// Corresponds to the `-D <path>` argument.
    pub fn data_dir(mut self, dir: PathBuf) -> Self {
        self.data_dir = Some(dir);
        self
    }

    /// Sets the OpenTTD configuration file path.
    ///
    /// Corresponds to the `-c <path>` argument.
    pub fn config_file(mut self, file: PathBuf) -> Self {
        self.config_file = Some(file);
        self
    }

    /// Sets the save game to load on startup.
    ///
    /// Corresponds to the `-g <path>` argument.
    pub fn save_game(mut self, save: PathBuf) -> Self {
        self.save_game = Some(save);
        self
    }

    /// Sets the multiplayer server to connect to.
    ///
    /// Corresponds to the `-n <host>:<port>` argument.
    /// The port must be a valid TCP port number.
    pub fn connect(mut self, host: String, port: u16) -> Self {
        self.connect_host = Some(host);
        self.connect_port = Some(port);
        self
    }

    /// Sets the multiplayer password.
    ///
    /// Corresponds to the `-p <password>` argument.
    /// Only used when [`connect`](ArgsBuilder::connect) is also set.
    pub fn password(mut self, pw: String) -> Self {
        self.connect_password = Some(pw);
        self
    }

    /// Enables or disables debug mode.
    ///
    /// When `true`, adds the `-d` argument.
    pub fn debug(mut self, yes: bool) -> Self {
        self.debug = yes;
        self
    }

    /// Sets the game resolution.
    ///
    /// Corresponds to the `-r <width>x<height>` argument.
    pub fn resolution(mut self, w: u32, h: u32) -> Self {
        self.resolution = Some((w, h));
        self
    }

    /// Enables or disables fullscreen mode.
    ///
    /// When `true`, adds the `-f` argument.
    pub fn fullscreen(mut self, yes: bool) -> Self {
        self.fullscreen = yes;
        self
    }

    /// Adds custom arguments that are appended at the end of the argument list.
    ///
    /// These arguments are passed through verbatim without any processing.
    /// They are placed after all standard arguments.
    pub fn custom(mut self, args: Vec<String>) -> Self {
        self.custom_args = args;
        self
    }

    /// Builds the final argument list.
    ///
    /// Produces a `Vec<String>` suitable for passing to
    /// [`std::process::Command::args`].
    ///
    /// The arguments are ordered as follows:
    /// 1. Data directory (`-D`)
    /// 2. Config file (`-c`)
    /// 3. Save game (`-g`)
    /// 4. Connection (`-n`)
    /// 5. Password (`-p`)
    /// 6. Debug mode (`-d`)
    /// 7. Resolution (`-r`)
    /// 8. Fullscreen (`-f`)
    /// 9. Custom arguments (in order)
    pub fn build(self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();

        if let Some(dir) = self.data_dir {
            args.push("-D".to_string());
            args.push(dir.to_string_lossy().to_string());
        }

        if let Some(file) = self.config_file {
            args.push("-c".to_string());
            args.push(file.to_string_lossy().to_string());
        }

        if let Some(save) = self.save_game {
            args.push("-g".to_string());
            args.push(save.to_string_lossy().to_string());
        }

        if let Some(host) = self.connect_host {
            let port = self.connect_port.unwrap_or(3979);
            args.push("-n".to_string());
            args.push(format!("{}:{}", host, port));
        }

        if let Some(pw) = self.connect_password {
            args.push("-p".to_string());
            args.push(pw);
        }

        if self.debug {
            args.push("-d".to_string());
        }

        if let Some((w, h)) = self.resolution {
            args.push("-r".to_string());
            args.push(format!("{}x{}", w, h));
        }

        if self.fullscreen {
            args.push("-f".to_string());
        }

        args.extend(self.custom_args);

        args
    }
}