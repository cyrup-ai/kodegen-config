use std::path::Path;

/// Display a path in the most concise human-readable format
///
/// **Shortening Strategy (in order of precedence):**
/// 1. If within git repository: relative to git root (e.g., `packages/kodegen-utils/Cargo.toml`)
/// 2. Else if within home directory: relative to `~` (e.g., `~/projects/external/file.txt`)
/// 3. Else: absolute path (e.g., `/usr/local/bin/tool`)
///
/// This function provides intelligent path shortening for user-facing output,
/// ensuring paths are as concise as possible while remaining unambiguous.
///
/// # Arguments
///
/// * `path` - The absolute path to display
/// * `git_root` - Optional git repository root from `ctx.git_root()`
///
/// # Returns
///
/// A shortened path string suitable for display to users.
///
/// # Examples
///
/// ```rust
/// use kodegen_config::shorten_path_for_display;
/// use std::path::Path;
///
/// // Within git repo: show relative to repo root
/// let path = Path::new("/Users/alice/project/src/main.rs");
/// let git_root = Some(Path::new("/Users/alice/project"));
/// assert_eq!(shorten_path_for_display(path, git_root), "src/main.rs");
///
/// // Outside git repo, within home: use ~ notation
/// let path = Path::new("/Users/alice/external/file.txt");
/// assert_eq!(shorten_path_for_display(path, None), "~/external/file.txt");
///
/// // Outside both git and home: show absolute
/// let path = Path::new("/usr/local/bin/tool");
/// assert_eq!(shorten_path_for_display(path, None), "/usr/local/bin/tool");
/// ```
pub fn shorten_path_for_display(path: &Path, git_root: Option<&Path>) -> String {
    // Strategy 1: Git root relative (highest priority)
    if let Some(root) = git_root {
        if let Ok(relative) = path.strip_prefix(root) {
            return relative.display().to_string();
        }
    }
    
    // Strategy 2: Home directory relative
    if let Some(home_dir) = dirs::home_dir() {
        if let Ok(relative) = path.strip_prefix(&home_dir) {
            // Format with tilde notation
            return format!("~/{}", relative.display());
        }
    }
    
    // Strategy 3: Absolute path (fallback)
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_git_root_shortening() {
        let path = PathBuf::from("/home/user/repo/src/main.rs");
        let git_root = PathBuf::from("/home/user/repo");
        
        let result = shorten_path_for_display(&path, Some(&git_root));
        assert_eq!(result, "src/main.rs");
    }

    #[test]
    fn test_home_directory_shortening() {
        // This test will only work if we can get a real home directory
        if let Some(home) = dirs::home_dir() {
            let path = home.join("external/file.txt");
            let result = shorten_path_for_display(&path, None);
            assert_eq!(result, "~/external/file.txt");
        }
    }

    #[test]
    fn test_absolute_path_fallback() {
        let path = PathBuf::from("/usr/local/bin/tool");
        let result = shorten_path_for_display(&path, None);
        assert_eq!(result, "/usr/local/bin/tool");
    }

    #[test]
    fn test_git_root_takes_precedence_over_home() {
        // If path is in both git repo AND home, git repo wins
        if let Some(home) = dirs::home_dir() {
            let git_root = home.join("projects/repo");
            let path = git_root.join("src/main.rs");
            
            let result = shorten_path_for_display(&path, Some(&git_root));
            assert_eq!(result, "src/main.rs");
        }
    }
}
