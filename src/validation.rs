//! Path traversal prevention for configuration file resolution
//!
//! Implements two-layer defense:
//! 1. Input validation - Reject path-like characters
//! 2. Canonicalization - Verify resolved paths stay within bounds
//!
//! References:
//! - CWE-22: https://cwe.mitre.org/data/definitions/22.html
//! - Rust PathBuf security: https://stackoverflow.com/questions/56366947/
//! - StackHawk Rust Path Traversal Guide: https://www.stackhawk.com/blog/rust-path-traversal-guide-example-and-prevention/

use log::warn;
use std::path::Path;

/// Windows reserved device names that cannot be used as filenames
#[cfg(target_os = "windows")]
const WINDOWS_RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", 
    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Validate that a name contains no path traversal sequences or malicious characters
///
/// **Validation Rules:**
/// 1. No path separators (`/` or `\`)
/// 2. No parent directory references (`..`)
/// 3. No null bytes (`\0`)
/// 4. Not empty or whitespace-only
/// 5. Not a hidden file (no leading `.`)
/// 6. (Windows only) Not a reserved device name
///
/// **Returns:** `Ok(())` if valid, `Err(String)` with detailed reason if invalid
///
/// # Examples
/// ```
/// assert!(validate_name("core").is_ok());
/// assert!(validate_name("my-toolset").is_ok());
/// assert!(validate_name("toolset_v2").is_ok());
/// 
/// assert!(validate_name("../etc/passwd").is_err());
/// assert!(validate_name("/absolute/path").is_err());
/// assert!(validate_name("foo/bar").is_err());
/// assert!(validate_name(".hidden").is_err());
/// ```
#[allow(dead_code)]
pub fn validate_name(name: &str) -> Result<(), String> {
    // Rule 1: Reject empty or whitespace-only names
    if name.trim().is_empty() {
        return Err("Name cannot be empty or whitespace-only".to_string());
    }

    // Rule 2: Reject path separators (cross-platform)
    if name.contains('/') {
        return Err(format!(
            "Name '{}' contains forward slash - path separators not allowed",
            name
        ));
    }
    
    if name.contains('\\') {
        return Err(format!(
            "Name '{}' contains backslash - path separators not allowed",
            name
        ));
    }

    // Rule 3: Reject parent directory traversal sequences
    if name.contains("..") {
        return Err(format!(
            "Name '{}' contains '..' - path traversal sequences not allowed",
            name
        ));
    }

    // Rule 4: Reject null bytes (can cause truncation attacks)
    if name.contains('\0') {
        return Err(format!(
            "Name '{}' contains null bytes - invalid character",
            name
        ));
    }

    // Rule 5: Reject hidden files (leading dot)
    if name.starts_with('.') {
        return Err(format!(
            "Name '{}' starts with '.' - hidden files not allowed",
            name
        ));
    }

    // Rule 6: Windows reserved device names
    #[cfg(target_os = "windows")]
    {
        let upper_name = name.to_uppercase();
        // Check both exact match and name-before-extension
        let base_name = if let Some(pos) = upper_name.find('.') {
            &upper_name[..pos]
        } else {
            &upper_name
        };
        
        if WINDOWS_RESERVED_NAMES.contains(&base_name) {
            return Err(format!(
                "Name '{}' is a Windows reserved device name",
                name
            ));
        }
    }

    Ok(())
}

/// Verify that a resolved path is within the expected base directory
///
/// Uses canonicalization to resolve symlinks and `.` / `..` sequences,
/// then verifies the canonical path starts with the canonical base.
///
/// **Security guarantees:**
/// - Resolves symlinks (prevents symlink-based traversal)
/// - Normalizes `.` and `..` sequences
/// - Handles case-insensitive filesystems correctly
/// - Returns `false` for broken symlinks or non-existent paths
///
/// **Returns:** `true` if path is within base, `false` otherwise
///
/// # Examples
/// ```
/// let base = Path::new("/home/user/.kodegen/toolset");
/// let safe = Path::new("/home/user/.kodegen/toolset/core.json");
/// let unsafe_path = Path::new("/etc/passwd");
/// 
/// assert!(verify_within_directory(safe, base));
/// assert!(!verify_within_directory(unsafe_path, base));
/// ```
#[allow(dead_code)]
pub fn verify_within_directory(resolved_path: &Path, base_dir: &Path) -> bool {
    // Attempt to canonicalize both paths
    let canonical_resolved = match resolved_path.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // Cannot canonicalize (broken symlink, non-existent, or permission denied)
            warn!(
                "Cannot canonicalize resolved path: {}",
                resolved_path.display()
            );
            return false;
        }
    };

    let canonical_base = match base_dir.canonicalize() {
        Ok(p) => p,
        Err(_) => {
            // Base directory doesn't exist or cannot be accessed
            warn!(
                "Cannot canonicalize base directory: {}",
                base_dir.display()
            );
            return false;
        }
    };

    // Verify resolved path starts with base directory
    if !canonical_resolved.starts_with(&canonical_base) {
        warn!(
            "Path traversal detected: '{}' escapes base directory '{}'",
            canonical_resolved.display(),
            canonical_base.display()
        );
        return false;
    }

    true
}