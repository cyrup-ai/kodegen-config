use anyhow::{anyhow, bail, Result};
use log::warn;
use std::path::PathBuf;

/// Check if the KODEGEN_ALLOW_CUSTOM_PATHS override is enabled
/// This allows power users to bypass validation in trusted environments
/// WARNING: This defeats the security purpose of this fix
fn is_custom_paths_allowed() -> bool {
    std::env::var("KODEGEN_ALLOW_CUSTOM_PATHS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

/// Check if path contains suspicious patterns that indicate attack attempts
fn has_suspicious_patterns(path: &std::path::Path) -> bool {
    let path_str = path.to_string_lossy();
    
    // Multiple consecutive dots (e.g., "....//")
    if path_str.contains("....") {
        return true;
    }
    
    // Null bytes
    if path_str.contains('\0') {
        return true;
    }
    
    // Control characters (except newline/tab)
    if path_str.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return true;
    }
    
    false
}

/// Validate environment variable path for security
/// Returns Ok(PathBuf) if path is safe, Err if path is malicious or invalid
fn validate_env_path(env_var_name: &str, path_str: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path_str);
    
    // Check for suspicious patterns first
    if has_suspicious_patterns(&path) {
        warn!(
            "Rejecting {}='{}': Contains suspicious patterns (null bytes, excessive dots, or control characters)",
            env_var_name, path_str
        );
        bail!("Path contains suspicious patterns");
    }
    
    // Attempt canonicalization to resolve symlinks and ".." sequences
    let canonical = match path.canonicalize() {
        Ok(p) => p,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Path doesn't exist yet - validate parent directory
            if let Some(parent) = path.parent() {
                if parent.exists() {
                    // Parent exists, canonicalize it and append the filename
                    let canonical_parent = parent.canonicalize().map_err(|e| {
                        warn!(
                            "Rejecting {}='{}': Failed to canonicalize parent directory: {}",
                            env_var_name, path_str, e
                        );
                        anyhow!("Failed to canonicalize parent directory: {}", e)
                    })?;
                    
                    let filename = path.file_name()
                        .ok_or_else(|| anyhow!("Invalid path: no filename"))?;
                    
                    canonical_parent.join(filename)
                } else {
                    warn!(
                        "Rejecting {}='{}': Parent directory does not exist",
                        env_var_name, path_str
                    );
                    bail!("Parent directory does not exist: {}", parent.display());
                }
            } else {
                warn!(
                    "Rejecting {}='{}': Path has no parent directory",
                    env_var_name, path_str
                );
                bail!("Path has no parent directory");
            }
        }
        Err(e) => {
            warn!(
                "Rejecting {}='{}': Failed to canonicalize: {}",
                env_var_name, path_str, e
            );
            bail!("Failed to canonicalize path: {}", e);
        }
    };
    
    // Ensure path is absolute
    if !canonical.is_absolute() {
        warn!(
            "Rejecting {}='{}': Path is not absolute after canonicalization",
            env_var_name, path_str
        );
        bail!("Path must be absolute");
    }
    
    // Platform-specific boundary validation
    #[cfg(unix)]
    {
        validate_unix_boundaries(&canonical, env_var_name, path_str)?;
    }
    
    #[cfg(target_os = "windows")]
    {
        validate_windows_boundaries(&canonical, env_var_name, path_str)?;
    }
    
    Ok(canonical)
}

/// Validate path boundaries for Unix/Linux/macOS
#[cfg(unix)]
fn validate_unix_boundaries(canonical: &std::path::Path, env_var_name: &str, original: &str) -> Result<()> {
    // Path must be under user's home directory OR /tmp OR /var/tmp (for testing)
    // Note: We canonicalize the boundary paths too because /tmp might be a symlink (e.g., to /private/tmp on macOS)
    let allowed = if let Some(home) = dirs::home_dir() {
        let canonical_home = home.canonicalize().unwrap_or(home);
        let tmp_canonical = PathBuf::from("/tmp").canonicalize().unwrap_or_else(|_| PathBuf::from("/tmp"));
        let var_tmp_canonical = PathBuf::from("/var/tmp").canonicalize().unwrap_or_else(|_| PathBuf::from("/var/tmp"));
        
        canonical.starts_with(&canonical_home)
            || canonical.starts_with(&tmp_canonical)
            || canonical.starts_with(&var_tmp_canonical)
    } else {
        // If no home directory, only allow /tmp and /var/tmp
        let tmp_canonical = PathBuf::from("/tmp").canonicalize().unwrap_or_else(|_| PathBuf::from("/tmp"));
        let var_tmp_canonical = PathBuf::from("/var/tmp").canonicalize().unwrap_or_else(|_| PathBuf::from("/var/tmp"));
        
        canonical.starts_with(&tmp_canonical) || canonical.starts_with(&var_tmp_canonical)
    };
    
    if !allowed {
        warn!(
            "Rejecting {}='{}': Path is outside allowed boundaries (must be under $HOME, /tmp, or /var/tmp)",
            env_var_name, original
        );
        bail!(
            "Path must be under user home directory, /tmp, or /var/tmp. Got: {}",
            canonical.display()
        );
    }
    
    Ok(())
}

/// Validate path boundaries for Windows
#[cfg(target_os = "windows")]
fn validate_windows_boundaries(canonical: &std::path::Path, env_var_name: &str, original: &str) -> Result<()> {
    let path_str = canonical.to_string_lossy();
    
    // Reject UNC paths (\\server\share)
    if path_str.starts_with(r"\\") {
        warn!(
            "Rejecting {}='{}': UNC paths are not allowed",
            env_var_name, original
        );
        bail!("UNC paths not allowed: {}", path_str);
    }
    
    // Reject device paths (\\?\ or \\.\)
    if path_str.starts_with(r"\\?\") || path_str.starts_with(r"\\.\") {
        warn!(
            "Rejecting {}='{}': Device paths are not allowed",
            env_var_name, original
        );
        bail!("Device paths not allowed: {}", path_str);
    }
    
    // Path must be under APPDATA or LOCALAPPDATA
    let allowed = std::env::var("APPDATA")
        .ok()
        .and_then(|appdata| PathBuf::from(appdata).canonicalize().ok())
        .map(|canonical_appdata| canonical.starts_with(&canonical_appdata))
        .unwrap_or(false)
        || std::env::var("LOCALAPPDATA")
            .ok()
            .and_then(|localappdata| PathBuf::from(localappdata).canonicalize().ok())
            .map(|canonical_local| canonical.starts_with(&canonical_local))
            .unwrap_or(false);
    
    if !allowed {
        warn!(
            "Rejecting {}='{}': Path must be under %APPDATA% or %LOCALAPPDATA%",
            env_var_name, original
        );
        bail!("Path must be under APPDATA or LOCALAPPDATA");
    }
    
    Ok(())
}

/// Get user-global config directory (XDG-compliant)
/// Unix/Linux: $XDG_CONFIG_HOME/kodegen (default: ~/.config/kodegen)
/// macOS: ~/Library/Application Support/kodegen
/// Windows: %APPDATA%\kodegen
#[cfg(target_os = "windows")]
pub fn user_config_dir() -> Result<PathBuf> {
    // Check for override flag first
    if is_custom_paths_allowed() {
        if let Ok(custom_path) = std::env::var("APPDATA") {
            warn!(
                "KODEGEN_ALLOW_CUSTOM_PATHS is enabled - bypassing validation for APPDATA (UNSAFE)"
            );
            return Ok(PathBuf::from(custom_path).join("kodegen"));
        }
    }
    
    // Try validated environment variable
    let validated = std::env::var("APPDATA")
        .ok()
        .and_then(|p| {
            match validate_env_path("APPDATA", &p) {
                Ok(validated) => Some(validated),
                Err(e) => {
                    warn!("Invalid APPDATA environment variable: {}. Falling back to system default.", e);
                    None
                }
            }
        });
    
    // Use validated path or fall back to dirs crate
    if let Some(validated_path) = validated {
        Ok(validated_path.join("kodegen"))
    } else {
        dirs::config_dir()
            .map(|d| d.join("kodegen"))
            .ok_or_else(|| anyhow!("Cannot determine config directory"))
    }
}

#[cfg(target_os = "macos")]
pub fn user_config_dir() -> Result<PathBuf> {
    // macOS doesn't typically use XDG_CONFIG_HOME, but respect it if set
    if is_custom_paths_allowed()
        && let Ok(custom_path) = std::env::var("XDG_CONFIG_HOME") {
            warn!(
                "KODEGEN_ALLOW_CUSTOM_PATHS is enabled - bypassing validation for XDG_CONFIG_HOME (UNSAFE)"
            );
            return Ok(PathBuf::from(custom_path).join("kodegen"));
        }
    
    let validated = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .and_then(|p| {
            match validate_env_path("XDG_CONFIG_HOME", &p) {
                Ok(validated) => Some(validated),
                Err(e) => {
                    warn!("Invalid XDG_CONFIG_HOME environment variable: {}. Falling back to system default.", e);
                    None
                }
            }
        });
    
    if let Some(validated_path) = validated {
        Ok(validated_path.join("kodegen"))
    } else {
        dirs::config_dir()
            .map(|d| d.join("kodegen"))
            .or_else(|| dirs::home_dir().map(|h| h.join("Library/Application Support/kodegen")))
            .ok_or_else(|| anyhow!("Cannot determine config directory"))
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn user_config_dir() -> Result<PathBuf> {
    if is_custom_paths_allowed() {
        if let Ok(custom_path) = std::env::var("XDG_CONFIG_HOME") {
            warn!(
                "KODEGEN_ALLOW_CUSTOM_PATHS is enabled - bypassing validation for XDG_CONFIG_HOME (UNSAFE)"
            );
            return Ok(PathBuf::from(custom_path).join("kodegen"));
        }
    }
    
    let validated = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .and_then(|p| {
            match validate_env_path("XDG_CONFIG_HOME", &p) {
                Ok(validated) => Some(validated),
                Err(e) => {
                    warn!("Invalid XDG_CONFIG_HOME environment variable: {}. Falling back to system default.", e);
                    None
                }
            }
        });
    
    if let Some(validated_path) = validated {
        Ok(validated_path.join("kodegen"))
    } else {
        dirs::config_dir()
            .map(|d| d.join("kodegen"))
            .ok_or_else(|| anyhow!("Cannot determine config directory"))
    }
}