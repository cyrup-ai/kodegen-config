use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

/// Type alias for the git root cache
/// Cache stores Option<PathBuf> where None means "not in a git repository"
type GitRootCache = parking_lot::RwLock<HashMap<PathBuf, Option<PathBuf>>>;

/// Global cache for git root discovery results
/// 
/// Keyed by current working directory to handle edge cases where process
/// changes working directory (rare but possible via std::env::set_current_dir).
/// Uses LazyLock for zero-cost initialization and RwLock for concurrent read access.
static GIT_ROOT_CACHE: std::sync::LazyLock<GitRootCache> =
    std::sync::LazyLock::new(|| parking_lot::RwLock::new(HashMap::new()));

/// Find the git repository root directory (cached)
/// 
/// This function caches results globally across all threads. The cache is keyed
/// by the current working directory, so it correctly handles the edge case where
/// a process changes directories.
/// 
/// **Performance:**
/// - First call for a directory: 1-50ms (filesystem walk via git2)
/// - Subsequent calls: <1μs (in-memory HashMap lookup with read lock)
/// 
/// **Thread Safety:** 
/// Uses double-checked locking pattern with RwLock for optimal concurrent performance.
///
/// # Errors
///
/// Returns an error if:
/// - Current directory cannot be determined (e.g., deleted or permission denied)
/// - Not in a git repository
/// - Git repository is invalid or corrupted
pub fn find_git_root() -> Result<PathBuf> {
    // Get current directory for cache key
    let current_dir = std::env::current_dir()
        .context("Failed to determine current directory")?;
    
    // Fast path: Check cache with read lock (concurrent reads allowed)
    {
        let cache = GIT_ROOT_CACHE.read();
        if let Some(cached) = cache.get(&current_dir) {
            // Return cached result, converting None to error
            return cached.clone()
                .context(format!(
                    "Not in a git repository (searched from: {})",
                    current_dir.display()
                ));
        }
    }
    
    // Slow path: Cache miss - acquire write lock and compute
    let mut cache = GIT_ROOT_CACHE.write();
    
    // Double-check: Another thread may have populated cache while we waited for write lock
    if let Some(cached) = cache.get(&current_dir) {
        return cached.clone()
            .context(format!(
                "Not in a git repository (searched from: {})",
                current_dir.display()
            ));
    }
    
    // Compute git root via filesystem walk
    let result = discover_git_root(&current_dir);
    
    // Store in cache - convert Result to Option for caching
    let cached_value = result.as_ref().ok().cloned();
    cache.insert(current_dir.clone(), cached_value);
    
    result
}

/// Internal function that performs the actual git repository discovery
/// 
/// This is separated from `find_git_root()` to keep caching logic isolated
/// from git discovery logic.
fn discover_git_root(current_dir: &PathBuf) -> Result<PathBuf> {
    let repo = git2::Repository::discover(current_dir)
        .context(format!(
            "Not in a git repository (searched from: {})",
            current_dir.display()
        ))?;

    repo.workdir()
        .map(|p| p.to_path_buf())
        .context("Git repository has no working directory (bare repository?)")
}

/// Clear the git root cache
/// 
/// This should rarely be needed in production. Use cases:
/// - After calling `std::env::set_current_dir()` to change working directory
/// - When `.git` directory is created/deleted (extremely rare)
/// - For debugging/troubleshooting cache behavior
/// 
/// **Note:** The cache will automatically repopulate on next `find_git_root()` call.
#[allow(dead_code)]
pub fn clear_git_root_cache() {
    GIT_ROOT_CACHE.write().clear();
}
