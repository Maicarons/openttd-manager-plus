//! Unit tests for the launcher core module.

use crate::args::ArgsBuilder;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// ArgsBuilder construction tests
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_default() {
    let builder = ArgsBuilder::new();
    let args = builder.build();
    assert!(args.is_empty(), "Default ArgsBuilder should produce no arguments");
}

#[test]
fn test_args_builder_default_trait() {
    let builder = ArgsBuilder::default();
    let args = builder.build();
    assert!(args.is_empty(), "Default ArgsBuilder should produce no arguments");
}

#[test]
fn test_args_builder_data_dir() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("/home/user/openttd"))
        .build();
    assert_eq!(args, vec!["-D", "/home/user/openttd"]);
}

#[test]
fn test_args_builder_config_file() {
    let args = ArgsBuilder::new()
        .config_file(PathBuf::from("/etc/openttd.cfg"))
        .build();
    assert_eq!(args, vec!["-c", "/etc/openttd.cfg"]);
}

#[test]
fn test_args_builder_save_game() {
    let args = ArgsBuilder::new()
        .save_game(PathBuf::from("/home/user/save.sav"))
        .build();
    assert_eq!(args, vec!["-g", "/home/user/save.sav"]);
}

#[test]
fn test_args_builder_connect() {
    let args = ArgsBuilder::new()
        .connect("server.example.com".to_string(), 3979)
        .build();
    assert_eq!(args, vec!["-n", "server.example.com:3979"]);
}

#[test]
fn test_args_builder_connect_custom_port() {
    let args = ArgsBuilder::new()
        .connect("127.0.0.1".to_string(), 12345)
        .build();
    assert_eq!(args, vec!["-n", "127.0.0.1:12345"]);
}

#[test]
fn test_args_builder_password() {
    let args = ArgsBuilder::new()
        .password("secret123".to_string())
        .build();
    assert_eq!(args, vec!["-p", "secret123"]);
}

#[test]
fn test_args_builder_debug() {
    let args = ArgsBuilder::new().debug(true).build();
    assert_eq!(args, vec!["-d"]);
}

#[test]
fn test_args_builder_debug_off() {
    let args = ArgsBuilder::new().debug(false).build();
    assert!(args.is_empty(), "Debug disabled should not produce -d");
}

#[test]
fn test_args_builder_resolution() {
    let args = ArgsBuilder::new().resolution(1920, 1080).build();
    assert_eq!(args, vec!["-r", "1920x1080"]);
}

#[test]
fn test_args_builder_resolution_non_standard() {
    let args = ArgsBuilder::new().resolution(3440, 1440).build();
    assert_eq!(args, vec!["-r", "3440x1440"]);
}

#[test]
fn test_args_builder_fullscreen() {
    let args = ArgsBuilder::new().fullscreen(true).build();
    assert_eq!(args, vec!["-f"]);
}

#[test]
fn test_args_builder_fullscreen_off() {
    let args = ArgsBuilder::new().fullscreen(false).build();
    assert!(args.is_empty(), "Fullscreen disabled should not produce -f");
}

#[test]
fn test_args_builder_custom() {
    let args = ArgsBuilder::new()
        .custom(vec!["-x".to_string(), "--extra".to_string()])
        .build();
    assert_eq!(args, vec!["-x", "--extra"]);
}

// ---------------------------------------------------------------------------
// ArgsBuilder.build() output format tests
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_order() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("/data"))
        .config_file(PathBuf::from("/cfg"))
        .save_game(PathBuf::from("/save"))
        .connect("host".to_string(), 3979)
        .password("pw".to_string())
        .debug(true)
        .resolution(800, 600)
        .fullscreen(true)
        .custom(vec!["--plugin".to_string()])
        .build();

    assert_eq!(
        args,
        vec![
            "-D", "/data",
            "-c", "/cfg",
            "-g", "/save",
            "-n", "host:3979",
            "-p", "pw",
            "-d",
            "-r", "800x600",
            "-f",
            "--plugin",
        ]
    );
}

#[test]
fn test_args_builder_connect_without_port() {
    let builder = ArgsBuilder::new()
        .connect("host".to_string(), 3979);
    let args = builder.build();
    // The port should be 3979 (default OpenTTD port)
    assert!(args.contains(&"-n".to_string()));
    let n_idx = args.iter().position(|a| a == "-n").unwrap();
    assert_eq!(args[n_idx + 1], "host:3979");
}

// ---------------------------------------------------------------------------
// Path escaping tests
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_path_with_spaces() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("C:\\Program Files\\OpenTTD Data"))
        .build();
    // Each argument is a separate string, so spaces in paths are naturally preserved
    assert_eq!(args.len(), 2);
    assert_eq!(args[0], "-D");
    assert_eq!(args[1], "C:\\Program Files\\OpenTTD Data");
}

#[test]
fn test_args_builder_path_with_unicode() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("/data/openttd/🚂/saves"))
        .build();
    assert_eq!(args, vec!["-D", "/data/openttd/🚂/saves"]);
}

#[test]
fn test_args_builder_path_with_special_chars() {
    let args = ArgsBuilder::new()
        .config_file(PathBuf::from("/home/user/openttd config/openttd-1.11.cfg"))
        .build();
    assert_eq!(args, vec!["-c", "/home/user/openttd config/openttd-1.11.cfg"]);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_empty_args() {
    let args = ArgsBuilder::new().build();
    assert!(args.is_empty());
}

#[test]
fn test_args_builder_custom_empty() {
    let args = ArgsBuilder::new()
        .custom(vec![])
        .build();
    assert!(args.is_empty());
}

#[test]
fn test_args_builder_data_dir_with_trailing_slash() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("/data/openttd/"))
        .build();
    assert_eq!(args, vec!["-D", "/data/openttd/"]);
}

#[test]
fn test_args_builder_relative_path() {
    let args = ArgsBuilder::new()
        .save_game(PathBuf::from("./saves/autosave.sav"))
        .build();
    assert_eq!(args, vec!["-g", "./saves/autosave.sav"]);
}

#[test]
fn test_args_builder_windows_path() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("D:\\Games\\OpenTTD\\data"))
        .build();
    assert_eq!(args, vec!["-D", "D:\\Games\\OpenTTD\\data"]);
}

#[test]
fn test_args_builder_all_options() {
    let args = ArgsBuilder::new()
        .data_dir(PathBuf::from("/data"))
        .config_file(PathBuf::from("/cfg"))
        .save_game(PathBuf::from("/save"))
        .connect("host".to_string(), 3979)
        .password("pw".to_string())
        .debug(true)
        .resolution(1920, 1080)
        .fullscreen(true)
        .custom(vec!["+extra".to_string()])
        .build();

    assert_eq!(args.len(), 15);
    assert!(args.contains(&"-D".to_string()));
    assert!(args.contains(&"-c".to_string()));
    assert!(args.contains(&"-g".to_string()));
    assert!(args.contains(&"-n".to_string()));
    assert!(args.contains(&"-p".to_string()));
    assert!(args.contains(&"-d".to_string()));
    assert!(args.contains(&"-r".to_string()));
    assert!(args.contains(&"-f".to_string()));
    assert!(args.contains(&"+extra".to_string()));
}

// ---------------------------------------------------------------------------
// Builder chaining / immutability tests
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_chaining_does_not_mutate_original() {
    // This tests that the builder methods take ownership (self) and return new
    // values, so calling build() consumes the builder.
    let builder = ArgsBuilder::new().data_dir(PathBuf::from("/data"));
    let args = builder.build();
    assert_eq!(args, vec!["-D", "/data"]);
    // builder is consumed here, so we can't use it again (that's the intended API)
}

#[test]
fn test_args_builder_reuse_after_build() {
    // Since ArgsBuilder implements Clone, users can clone before building
    let builder = ArgsBuilder::new()
        .data_dir(PathBuf::from("/data"))
        .debug(true);

    let args1 = builder.clone().build();
    let args2 = builder.build();

    assert_eq!(args1, args2);
    assert_eq!(args1, vec!["-D", "/data", "-d"]);
}

// ---------------------------------------------------------------------------
// Password argument tests
// ---------------------------------------------------------------------------

#[test]
fn test_args_builder_password_with_empty_string() {
    let args = ArgsBuilder::new()
        .password("".to_string())
        .build();
    assert_eq!(args, vec!["-p", ""]);
}

#[test]
fn test_args_builder_password_with_special_chars() {
    let args = ArgsBuilder::new()
        .password("p@ssw0rd!\"#$%&'".to_string())
        .build();
    assert_eq!(args, vec!["-p", "p@ssw0rd!\"#$%&'"]);
}