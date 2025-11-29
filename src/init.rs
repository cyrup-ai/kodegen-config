use anyhow::{anyhow, Result};
use ignore::gitignore::GitignoreBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

/// Initialize directory structures for both local and user config
pub fn create_directory_structure() -> Result<()> {
    create_user_structure()?;
    // Create local structure only if in git repo (ignore error if not)
    if let Ok(local_dir) = crate::KodegenConfig::local_config_dir() {
        create_local_structure(&local_dir)?;
    }
    Ok(())
}

/// Create user-global directory structure
fn create_user_structure() -> Result<()> {
    let config_dir = crate::KodegenConfig::user_config_dir()?;
    let state_dir = crate::KodegenConfig::state_dir()?;
    let data_dir = crate::KodegenConfig::data_dir()?;
    let log_dir = crate::KodegenConfig::log_dir()?;

    // Create config subdirectories
    fs::create_dir_all(config_dir.join("toolset"))?;
    fs::create_dir_all(config_dir.join("claude"))?;

    // Create state directory (for PIDs, sockets, runtime state)
    fs::create_dir_all(&state_dir)?;

    // Create log directory (for .log files)
    fs::create_dir_all(&log_dir)?;

    // Create data subdirectories
    fs::create_dir_all(data_dir.join("stats"))?;
    fs::create_dir_all(data_dir.join("memory"))?;

    // Create .gitignore if it doesn't exist
    let gitignore_path = config_dir.join(".gitignore");
    if !gitignore_path.exists() {
        fs::write(gitignore_path, "*.log\n*.tmp\n*.cache\n")?;
    }

    Ok(())
}

/// Create local .kodegen directory structure
fn create_local_structure(local_dir: &Path) -> Result<()> {
    // Validate input: local_dir should end with ".kodegen"
    if local_dir.file_name() != Some(std::ffi::OsStr::new(".kodegen")) {
        log::warn!(
            "Unexpected local_dir path structure: {}. Expected path ending with '.kodegen'",
            local_dir.display()
        );
    }

    // Create .kodegen subdirectories
    fs::create_dir_all(local_dir.join("toolset"))?;
    fs::create_dir_all(local_dir.join("claude"))?;

    // Git root must be parent of .kodegen - use ok_or_else pattern
    let git_root = local_dir.parent().ok_or_else(|| {
        anyhow!(
            "Cannot determine git root: local_dir has no parent directory ({}). \
             This indicates a bug in git repository discovery",
            local_dir.display()
        )
    })?;

    add_to_gitignore(git_root)?;

    Ok(())
}

/// Add .kodegen to .gitignore if not already present
///
/// Uses semantic gitignore pattern matching to detect if .kodegen is already
/// ignored by any pattern (e.g., `.kodegen/`, `**/.kodegen/`, `/.kodegen/`).
/// 
/// This prevents false positives from substring matches against comments,
/// similar directory names, or unrelated patterns.
///
/// Security: This function explicitly rejects symbolic links to prevent
/// arbitrary file read/write attacks (CWE-61). It uses atomic writes
/// via temporary files to prevent race conditions (CWE-362).
fn add_to_gitignore(git_root: &Path) -> Result<()> {
    let gitignore_path = git_root.join(".gitignore");
    
    // SECURITY: Check if .gitignore exists and verify it's not a symlink
    // Using symlink_metadata() instead of metadata() - crucial difference:
    // - symlink_metadata() does NOT follow symlinks (uses lstat on Unix)
    // - metadata() DOES follow symlinks (uses stat on Unix)
    if gitignore_path.exists() {
        let metadata = fs::symlink_metadata(&gitignore_path)?;
        
        // Reject symbolic links
        if metadata.file_type().is_symlink() {
            // Log security event
            log::warn!(
                "Security: Refusing to modify .gitignore - it is a symbolic link: {}",
                gitignore_path.display()
            );
            
            return Err(anyhow::anyhow!(
                "Security: .gitignore is a symbolic link (refusing to modify): {}\n\
                 Remove the symlink and create a regular file instead.",
                gitignore_path.display()
            ));
        }
        
        // Reject non-regular files (directories, devices, etc.)
        if !metadata.file_type().is_file() {
            return Err(anyhow::anyhow!(
                ".gitignore exists but is not a regular file: {}",
                gitignore_path.display()
            ));
        }
    }
    
    // Read existing content (now safe - we verified it's a regular file)
    let content = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };
    
    // Build gitignore matcher from existing .gitignore file using semantic pattern matching
    let mut builder = GitignoreBuilder::new(git_root);
    if gitignore_path.exists() {
        builder.add(&gitignore_path);
    }
    let gitignore = builder.build()?;
    
    // Test if .kodegen directory would be ignored using semantic pattern matching
    // We test a hypothetical file inside .kodegen to see if the directory is ignored
    // This correctly handles all gitignore pattern variations:
    // - .kodegen/ (exact match)
    // - .kodegen (without trailing slash)
    // - /.kodegen/ (root-only pattern)
    // - **/.kodegen/ (any subdirectory)
    // - .kodegen/** (everything inside .kodegen)
    let test_path = git_root.join(".kodegen/test.txt");
    let is_ignored = gitignore.matched(&test_path, false).is_ignore();
    
    // Only add .kodegen/ entry if it's not already semantically ignored
    if !is_ignored {
        // Use atomic write pattern from kodegend/src/install/binary_staging.rs
        // Create temporary file in the same directory as target
        // This ensures atomic replacement and prevents partial writes
        let mut temp_file = NamedTempFile::new_in(git_root)?;
        
        // Write existing content
        temp_file.write_all(content.as_bytes())?;
        
        // Add newline before .kodegen entry if content doesn't end with one
        if !content.is_empty() && !content.ends_with('\n') {
            temp_file.write_all(b"\n")?;
        }
        
        // Add .kodegen entry
        temp_file.write_all(b".kodegen/\n")?;
        
        // Atomically replace .gitignore
        // persist() performs atomic rename (mv on Unix, MoveFileEx on Windows)
        // This prevents:
        // - Race conditions (CWE-362)
        // - Partial writes from crashes
        // - TOCTOU (Time-of-check-time-of-use) vulnerabilities
        temp_file.persist(&gitignore_path)?;
        
        log::info!("Added .kodegen/ to .gitignore: {}", gitignore_path.display());
    }
    
    Ok(())
}
