//! Configuration integration tests
//! Tests the configuration system integration with analyzers

use analyzer::core::{Config, TechStack};
use analyzer::plugins;

#[test]
fn test_config_loading() {
    let config = Config::load(std::path::Path::new(".")).unwrap();
    assert!(!config.commands.is_empty());
}

#[test]
fn test_registry_with_config() {
    let config = Config::load(std::path::Path::new(".")).unwrap();
    let registry = plugins::create_registry_with_config(Some(config));

    // Verify registry has analyzers
    assert!(!registry.list().is_empty());

    // Verify cargo analyzer is registered
    let analyzer = registry.get(TechStack::Cargo);
    assert!(analyzer.is_some());
}

#[test]
fn test_registry_without_config() {
    let registry = plugins::create_registry_with_config(None);

    // Verify registry has analyzers
    assert!(!registry.list().is_empty());

    // Verify cargo analyzer is registered
    let analyzer = registry.get(TechStack::Cargo);
    assert!(analyzer.is_some());
}

#[test]
fn test_config_get_command_exec() {
    let config = Config::load(std::path::Path::new(".")).unwrap();

    // Test getting cargo check command
    let cmd = config.get_command_exec("cargo", "check");
    assert!(cmd.is_some());
    assert_eq!(cmd.unwrap(), "cargo check");

    // Test getting npm lint command
    let cmd = config.get_command_exec("npm", "lint");
    assert!(cmd.is_some());
    assert_eq!(cmd.unwrap(), "npm run lint");

    // Test non-existent command
    let cmd = config.get_command_exec("cargo", "nonexistent");
    assert!(cmd.is_none());
}

#[test]
fn test_config_merge() {
    let mut base = Config::load(std::path::Path::new(".")).unwrap();

    let mut override_config = Config::default();
    override_config.commands.insert(
        "check".to_string(),
        analyzer::core::config::CommandConfig {
            exec: "cargo check --all-targets".to_string(),
            description: Some("Custom check".to_string()),
            tech_stacks: vec!["cargo".to_string()],
            enabled: true,
        },
    );

    base.merge(override_config);

    let check_cmd = base.commands.get("check");
    assert_eq!(check_cmd.unwrap().exec, "cargo check --all-targets");
}

#[test]
fn test_tech_stack_override() {
    let mut config = Config::load(std::path::Path::new(".")).unwrap();

    // Add tech_stack specific override
    let mut stack_config = analyzer::core::config::TechStackConfig::default();
    stack_config.commands.insert(
        "check".to_string(),
        analyzer::core::config::CommandConfig {
            exec: "cargo check --all-targets --all-features".to_string(),
            description: Some("Custom check".to_string()),
            tech_stacks: vec![],
            enabled: true,
        },
    );
    config.tech_stacks.insert("cargo".to_string(), stack_config);

    // Tech_stack specific command should take precedence
    let cmd = config.get_command_exec("cargo", "check");
    assert_eq!(cmd, Some("cargo check --all-targets --all-features".to_string()));
}
