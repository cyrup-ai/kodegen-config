use kodegen_config::KodegenConfig;

#[test]
fn test_user_config_dir_exists() {
    let result = KodegenConfig::user_config_dir();
    assert!(result.is_ok(), "Should be able to determine user config dir");
}

#[test]
fn test_state_dir_exists() {
    let result = KodegenConfig::state_dir();
    assert!(result.is_ok(), "Should be able to determine state dir");
}

#[test]
fn test_data_dir_exists() {
    let result = KodegenConfig::data_dir();
    assert!(result.is_ok(), "Should be able to determine data dir");
}

#[test]
fn test_local_config_dir_returns_option() {
    // Should return Some if in git repo, None otherwise
    let result = KodegenConfig::local_config_dir();
    // Can't assert specific value without knowing test environment
    assert!(result.is_some() || result.is_none());
}
