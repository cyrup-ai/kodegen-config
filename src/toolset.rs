use anyhow::{anyhow, Result};
use std::path::PathBuf;

/// Resolve toolset file path with local > user precedence
///
/// **Search order**:
/// 1. `${git_root}/.kodegen/toolset/{name}.json`
/// 2. `$XDG_CONFIG_HOME/kodegen/toolset/{name}.json`
///
/// # Errors
///
/// Returns an error if the toolset file is not found in either location.
/// The error message includes all searched paths to aid debugging.
pub fn resolve(name: &str) -> Result<PathBuf> {
    let filename = format!("{}.json", name);
    let mut searched_paths = Vec::new();

    // Check local .kodegen/toolset/ first
    if let Ok(local_dir) = crate::KodegenConfig::local_config_dir() {
        let local_path = local_dir.join("toolset").join(&filename);
        searched_paths.push(local_path.display().to_string());
        if let Some(path) = crate::try_resolve_in_dir(&local_dir, "toolset", &filename) {
            return Ok(path);
        }
    }

    // Check user global config/toolset/
    let user_dir = crate::KodegenConfig::user_config_dir()?;
    let user_path = user_dir.join("toolset").join(&filename);
    searched_paths.push(user_path.display().to_string());
    if let Some(path) = crate::try_resolve_in_dir(&user_dir, "toolset", &filename) {
        return Ok(path);
    }

    // Not found - provide helpful error with all searched locations
    Err(anyhow!(
        "Toolset '{}' not found. Searched:\n  {}",
        name,
        searched_paths.join("\n  ")
    ))
}
