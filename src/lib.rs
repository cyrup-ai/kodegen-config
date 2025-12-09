//! # kodegen-config
//!
//! Centralized configuration path resolution for KODEGEN.ᴀɪ
//!
//! ## Features
//!
//! - **Cross-platform**: Windows, macOS, Unix/Linux support via XDG Base Directory spec
//! - **Dual config support**: Git-local (`.kodegen/`) and user-global (`~/.config/kodegen/`)
//! - **Per-file precedence**: Config files resolved by checking local first, then user
//! - **Auto-initialization**: Creates directory structures on first use
//! - **Rich error context**: All operations return `Result<T>` with detailed error messages
//!
//! ## Error Handling Pattern
//!
//! All path resolution functions return `Result<PathBuf>` for consistency:
//!
//! **Root directories:**
//! - [`user_config_dir()`](KodegenConfig::user_config_dir) - User-global root directory (~/.config/kodegen)
//! - [`local_config_dir()`](KodegenConfig::local_config_dir) - Git workspace-local config directory (.kodegen/)
//!
//! **Subdirectories (all under root):**
//! - [`config_dir()`](KodegenConfig::config_dir) - Configuration files (root/config/)
//! - [`toolset_dir()`](KodegenConfig::toolset_dir) - Tool definitions (root/toolset/)
//! - [`state_dir()`](KodegenConfig::state_dir) - Runtime state: PIDs, sockets (root/state/)
//! - [`log_dir()`](KodegenConfig::log_dir) - Log files (root/logs/)
//! - [`data_dir()`](KodegenConfig::data_dir) - Persistent data: DB, certs (root/data/)
//! - [`cache_dir()`](KodegenConfig::cache_dir) - Temporary cache: builds, downloads (root/cache/)
//! - [`bin_dir()`](KodegenConfig::bin_dir) - Binaries (root/bin/)
//!
//! **File resolution:**
//! - [`resolve_toolset()`](KodegenConfig::resolve_toolset) - Resolve toolset file with precedence
//! - [`resolve_config_file()`](KodegenConfig::resolve_config_file) - Resolve config file with precedence
//!
//! This uniform `Result` pattern provides:
//! 1. **Consistency** - All similar operations use the same error handling pattern
//! 2. **Rich error context** - Errors explain what failed and where the system searched
//! 3. **Flexible handling** - Callers can propagate (`?`), unwrap, or convert to `Option` via `.ok()`
//!
//! ## Usage Examples
//!
//! ### Error Propagation (Recommended)
//!
//! ```rust
//! use kodegen_config::KodegenConfig;
//! use anyhow::Result;
//!
//! fn my_function() -> Result<()> {
//!     // Propagate errors with ?
//!     let user_config = KodegenConfig::user_config_dir()?;
//!     let local_config = KodegenConfig::local_config_dir()?;
//!     let toolset = KodegenConfig::resolve_toolset("core")?;
//!     
//!     println!("User config: {}", user_config.display());
//!     println!("Local config: {}", local_config.display());
//!     println!("Toolset: {}", toolset.display());
//!     Ok(())
//! }
//! ```
//!
//! ### Handling Missing Files as Non-Errors
//!
//! ```rust
//! use kodegen_config::KodegenConfig;
//! use anyhow::Result;
//!
//! fn try_load_local_config() -> Result<()> {
//!     // Convert Result to Option if you want to treat "not found" as non-error
//!     if let Ok(local_dir) = KodegenConfig::local_config_dir() {
//!         println!("In git repo, local config: {}", local_dir.display());
//!     } else {
//!         println!("Not in git repo, that's fine");
//!     }
//!     
//!     // Or use unwrap_or for fallback
//!     let config_dir = KodegenConfig::local_config_dir()
//!         .unwrap_or_else(|_| KodegenConfig::user_config_dir().unwrap());
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Inspecting Error Details
//!
//! ```rust
//! use kodegen_config::KodegenConfig;
//!
//! match KodegenConfig::resolve_toolset("nonexistent") {
//!     Ok(path) => println!("Found: {}", path.display()),
//!     Err(e) => {
//!         // Error message includes all searched paths
//!         eprintln!("Error: {}", e);
//!         // Output: "Toolset 'nonexistent' not found. Searched:
//!         //           /repo/.kodegen/toolset/nonexistent.json
//!         //           /home/user/.config/kodegen/toolset/nonexistent.json"
//!     }
//! }
//! ```

use anyhow::Result;
use std::path::{Path, PathBuf};

mod validation;
mod git;
mod init;
pub(crate) mod platform;  // Keep for user_config_dir implementation
mod toolset;
mod path_display;

pub mod constants;

pub use path_display::shorten_path_for_display;

// Re-export all constants for convenience
pub use constants::*;

// ============================================================================
// Infrastructure HTTP Headers
// ============================================================================
// These headers pass infrastructure context from kodegen stdio server to HTTP
// backend servers. Used for CWD tracking, git root detection, and connection-
// scoped resource isolation.

/// Header containing the connection ID for this stdio connection instance.
/// Used by backend servers for connection-scoped resource isolation (terminals, browsers, etc.)
pub const X_KODEGEN_CONNECTION_ID: &str = "x-kodegen-connection-id";

/// Header containing the current working directory from which kodegen was spawned.
/// Used by backend servers for path resolution and as default CWD for operations.
pub const X_KODEGEN_PWD: &str = "x-kodegen-pwd";

/// Header containing the git repository root directory.
/// Used for repository-aware operations and path resolution.
pub const X_KODEGEN_GITROOT: &str = "x-kodegen-gitroot";

/// Try to resolve a file within a directory with TOCTOU-resistant canonicalization
///
/// This function eliminates the TOCTOU race condition by avoiding explicit `.exists()`
/// checks. Instead, it attempts to canonicalize the file path, which:
/// 1. Implicitly checks existence (fails if file doesn't exist)
/// 2. Resolves all symlinks (prevents symlink attacks)
/// 3. Returns absolute, normalized path
///
/// After canonicalization, the function validates that the resolved path is within
/// the expected directory to prevent symlink-based directory traversal attacks.
///
/// # Arguments
///
/// * `base_dir` - The base configuration directory (local or user)
/// * `subdir` - Subdirectory within base (e.g., "toolset", "" for root)
/// * `filename` - The filename to resolve
///
/// # Returns
///
/// * `Some(PathBuf)` - Canonical path if file exists and is within expected directory
/// * `None` - If file doesn't exist, can't be accessed, or is outside expected directory
///
/// # Security
///
/// - **TOCTOU Mitigation**: No explicit existence check - canonicalize() implicitly validates
/// - **Symlink Protection**: All symlinks are resolved and validated against base directory
/// - **Path Traversal Prevention**: Canonical path must start with canonical base directory
///
/// # Notes
///
/// A small TOCTOU window still exists between this function returning the canonical path
/// and the caller actually accessing the file. This is unavoidable given the API design
/// that returns a path for later use. Callers should handle potential I/O errors gracefully.
pub(crate) fn try_resolve_in_dir(base_dir: &Path, subdir: &str, filename: &str) -> Option<PathBuf> {
    // Build the full path to search
    let search_path = if subdir.is_empty() {
        base_dir.join(filename)
    } else {
        base_dir.join(subdir).join(filename)
    };
    
    // Build the base directory for bounds checking
    let base_path = if subdir.is_empty() {
        base_dir.to_path_buf()
    } else {
        base_dir.join(subdir)
    };
    
    // Try to canonicalize the file path
    // This fails if:
    // - File doesn't exist
    // - File is inaccessible (permissions)
    // - Any parent directory doesn't exist
    // This implicitly checks existence, eliminating the TOCTOU .exists() call
    let canonical_file = search_path.canonicalize().ok()?;
    
    // Try to canonicalize the base directory for secure comparison
    // If this fails, the directory doesn't exist, so the file can't be valid
    let canonical_base = base_path.canonicalize().ok()?;
    
    // Verify the canonical file path is within the canonical base directory
    // This prevents symlink attacks where the file is actually outside the expected directory
    if canonical_file.starts_with(&canonical_base) {
        Some(canonical_file)
    } else {
        // File resolved to a location outside the expected directory
        // This indicates a symlink attack or path traversal attempt
        None
    }
}

/// Main configuration path resolver
pub struct KodegenConfig;

impl KodegenConfig {
    /// Get user-global config directory (XDG-compliant)
    ///
    /// **Platform paths**:
    /// - Unix/Linux: `$XDG_CONFIG_HOME/kodegen` (default: `~/.config/kodegen`)
    /// - macOS: `~/Library/Application Support/kodegen`
    /// - Windows: `%APPDATA%\kodegen`
    ///
    /// This ALWAYS returns the user-global config directory, never the local `.kodegen/`.
    pub fn user_config_dir() -> Result<PathBuf> {
        platform::user_config_dir()
    }

    /// Get git workspace-local config directory
    ///
    /// **Returns**: `${git_root}/.kodegen`
    ///
    /// This ONLY returns the local `.kodegen/` directory, never the user config.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Not in a git repository
    /// - Current directory cannot be determined
    /// - Git repository is invalid or corrupted
    pub fn local_config_dir() -> Result<PathBuf> {
        git::find_git_root().map(|root| root.join(".kodegen"))
    }

    /// Get config subdirectory (for daemon configuration files)
    ///
    /// **Returns**: `{root}/config/`
    ///
    /// Example: `~/.config/kodegen/config/`
    pub fn config_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("config"))
    }

    /// Get toolset subdirectory (for tool definitions)
    ///
    /// **Returns**: `{root}/toolset/`
    ///
    /// Example: `~/.config/kodegen/toolset/`
    pub fn toolset_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("toolset"))
    }

    /// Get state subdirectory (for PIDs, sockets, runtime state)
    ///
    /// **Returns**: `{root}/state/`
    ///
    /// Example: `~/.config/kodegen/state/`
    pub fn state_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("state"))
    }

    /// Get log subdirectory (for .log files)
    ///
    /// **Returns**: `{root}/logs/`
    ///
    /// Example: `~/.config/kodegen/logs/`
    pub fn log_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("logs"))
    }

    /// Get data subdirectory (for databases, stats, caches, certificates)
    ///
    /// **Returns**: `{root}/data/`
    ///
    /// Example: `~/.config/kodegen/data/`
    pub fn data_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("data"))
    }

    /// Get bin subdirectory (for binary storage before symlinking)
    ///
    /// **Returns**: `{root}/bin/`
    ///
    /// Example: `~/.config/kodegen/bin/`
    pub fn bin_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("bin"))
    }

    /// Get cache subdirectory (for temporary build artifacts, downloads, Chrome cache)
    ///
    /// **Returns**: `{root}/cache/`
    ///
    /// Example: `~/.config/kodegen/cache/`
    pub fn cache_dir() -> Result<PathBuf> {
        Ok(Self::user_config_dir()?.join("cache"))
    }

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
    pub fn resolve_toolset(name: &str) -> Result<PathBuf> {
        toolset::resolve(name)
    }

    /// Resolve config file path with local > user precedence
    ///
    /// **Search order**:
    /// 1. `${git_root}/.kodegen/{filename}`
    /// 2. `$XDG_CONFIG_HOME/kodegen/{filename}`
    ///
    /// # Errors
    ///
    /// Returns an error if the config file is not found in either location.
    /// The error message includes all searched paths to aid debugging.
    pub fn resolve_config_file(filename: &str) -> Result<PathBuf> {
        let mut searched_paths = Vec::new();

        // Check local first
        if let Ok(local_dir) = Self::local_config_dir() {
            let local_path = local_dir.join(filename);
            searched_paths.push(local_path.display().to_string());
            if let Some(path) = try_resolve_in_dir(&local_dir, "", filename) {
                return Ok(path);
            }
        }

        // Check user global
        let user_dir = Self::user_config_dir()?;
        let user_path = user_dir.join(filename);
        searched_paths.push(user_path.display().to_string());
        if let Some(path) = try_resolve_in_dir(&user_dir, "", filename) {
            return Ok(path);
        }

        Err(anyhow::anyhow!(
            "Config file '{}' not found. Searched:\n  {}",
            filename,
            searched_paths.join("\n  ")
        ))
    }

    /// Initialize directory structures for both local and user config
    ///
    /// Creates:
    /// - User config: `toolset/`, `claude/` subdirectories + `.gitignore`
    /// - User state: `logs/` subdirectory
    /// - User data: `stats/`, `memory/` subdirectories
    /// - Local config (if in git repo): `toolset/`, `claude/` + adds to `.gitignore`
    pub fn init_structure() -> Result<()> {
        init::create_directory_structure()
    }
}
