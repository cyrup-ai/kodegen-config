//! Centralized constants for kodegen infrastructure
//!
//! This module provides the single source of truth for:
//! - Tool names (used in tool metadata)
//! - Category names (used in tool metadata and routing)
//! - Port assignments (used by HTTP MCP servers)
//! - Category-to-port mappings
//!
//! All references to these values MUST use these constants - no hardcoded strings/numbers.

// ============================================================================
// CATEGORY TYPE
// ============================================================================

/// Structured category definition with name and icon
///
/// This ensures every category has both a name and an icon at compile time.
/// Used by ToolArgs trait to provide default icon() implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Category {
    pub name: &'static str,
    pub icon: char,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

// ============================================================================
// CATEGORY CONSTANTS
// ============================================================================

/// Browser automation and web interaction
pub const CATEGORY_BROWSER: &Category = &Category {
    name: "browser",
    icon: 'Ƅ',  // LATIN CAPITAL LETTER TONE SIX
};

/// Candle-based local LLM agent (serves memory tools)
pub const CATEGORY_CANDLE_AGENT: &Category = &Category {
    name: "candle_agent",
    icon: 'Ⲵ',  // COPTIC CAPITAL LETTER OLD COPTIC AIN
};

/// Web crawling and indexing with Tantivy
pub const CATEGORY_CITESCRAPE: &Category = &Category {
    name: "citescrape",
    icon: '⚚',  // STAFF OF HERMES
};

/// Claude sub-agent delegation
pub const CATEGORY_CLAUDE_AGENT: &Category = &Category {
    name: "claude_agent",
    icon: 'Ⲵ',  // COPTIC CAPITAL LETTER OLD COPTIC AIN
};

/// Configuration value management
pub const CATEGORY_CONFIG: &Category = &Category {
    name: "config",
    icon: '⚙',  // GEAR
};

/// Database operations and schema inspection
pub const CATEGORY_DATABASE: &Category = &Category {
    name: "database",
    icon: '⛁',  // WHITE DRAUGHTS KING
};

/// File system operations
pub const CATEGORY_FILESYSTEM: &Category = &Category {
    name: "filesystem",
    icon: '⚒',  // HAMMER AND PICK
};

/// Git version control operations
pub const CATEGORY_GIT: &Category = &Category {
    name: "git",
    icon: '⛙',  // WHITE LEFT LANE MERGE
};

/// GitHub API operations
pub const CATEGORY_GITHUB: &Category = &Category {
    name: "github",
    icon: '⇅',  // UTF 113
};

/// Tool usage statistics and introspection
pub const CATEGORY_INTROSPECTION: &Category = &Category {
    name: "introspection",
    icon: '⚝',  // STAR WITH INSIDE LINES
};

/// Memory and knowledge management (served by candle-agent)
pub const CATEGORY_MEMORY: &Category = &Category {
    name: "memory",
    icon: '⚿',  // SQUARED KEY
};

/// Process management
pub const CATEGORY_PROCESS: &Category = &Category {
    name: "process",
    icon: '♆',  // NEPTUNE
};

/// Prompt template management
pub const CATEGORY_PROMPT: &Category = &Category {
    name: "prompt",
    icon: '⚑',  // BLACK FLAG
};

/// Reasoner tool backend
pub const CATEGORY_REASONER: &Category = &Category {
    name: "reasoner",
    icon: '☫',  // FARSI SYMBOL
};

/// Sequential thinking tool backend
pub const CATEGORY_SEQUENTIAL_THINKING: &Category = &Category {
    name: "sequential_thinking",
    icon: '⚛',  // ATOM
};

/// Terminal command execution
pub const CATEGORY_TERMINAL: &Category = &Category {
    name: "terminal",
    icon: '⛩',  // SHINTO SHRINE
};

// ============================================================================
// TOOL NAME CONSTANTS
// ============================================================================

// Memory tools (served by candle-agent)
pub const MEMORY_MEMORIZE: &str = "memory_memorize";
pub const MEMORY_RECALL: &str = "memory_recall";
pub const MEMORY_LIST_LIBRARIES: &str = "memory_list_libraries";
pub const MEMORY_CHECK_MEMORIZE_STATUS: &str = "memory_check_memorize_status";

// Claude agent tools
pub const CLAUDE_AGENT: &str = "claude_agent";

// Filesystem tools
pub const FS_CREATE_DIRECTORY: &str = "fs_create_directory";
pub const FS_DELETE_DIRECTORY: &str = "fs_delete_directory";
pub const FS_DELETE_FILE: &str = "fs_delete_file";
pub const FS_EDIT_BLOCK: &str = "fs_edit_block";
pub const FS_GET_FILE_INFO: &str = "fs_get_file_info";
pub const FS_LIST_DIRECTORY: &str = "fs_list_directory";
pub const FS_MOVE_FILE: &str = "fs_move_file";
pub const FS_READ_FILE: &str = "fs_read_file";
pub const FS_READ_MULTIPLE_FILES: &str = "fs_read_multiple_files";
pub const FS_SEARCH: &str = "fs_search";
pub const FS_WRITE_FILE: &str = "fs_write_file";

// Git tools
pub const GIT_ADD: &str = "git_add";
pub const GIT_BRANCH_CREATE: &str = "git_branch_create";
pub const GIT_BRANCH_DELETE: &str = "git_branch_delete";
pub const GIT_BRANCH_LIST: &str = "git_branch_list";
pub const GIT_BRANCH_RENAME: &str = "git_branch_rename";
pub const GIT_CHERRY_PICK: &str = "git_cherry_pick";
pub const GIT_CHECKOUT: &str = "git_checkout";
pub const GIT_CLONE: &str = "git_clone";
pub const GIT_COMMIT: &str = "git_commit";
pub const GIT_CONFIG_GET: &str = "git_config_get";
pub const GIT_CONFIG_SET: &str = "git_config_set";
pub const GIT_DIFF: &str = "git_diff";
pub const GIT_DISCOVER: &str = "git_discover";
pub const GIT_FETCH: &str = "git_fetch";
pub const GIT_HISTORY: &str = "git_history";
pub const GIT_INIT: &str = "git_init";
pub const GIT_LOG: &str = "git_log";
pub const GIT_MERGE: &str = "git_merge";
pub const GIT_OPEN: &str = "git_open";
pub const GIT_PULL: &str = "git_pull";
pub const GIT_PUSH: &str = "git_push";
pub const GIT_REBASE: &str = "git_rebase";
pub const GIT_REMOTE_ADD: &str = "git_remote_add";
pub const GIT_REMOTE_LIST: &str = "git_remote_list";
pub const GIT_REMOTE_REMOVE: &str = "git_remote_remove";
pub const GIT_RESET: &str = "git_reset";
pub const GIT_REVERT: &str = "git_revert";
pub const GIT_SHOW: &str = "git_show";
pub const GIT_STASH: &str = "git_stash";
pub const GIT_STASH_APPLY: &str = "git_stash_apply";
pub const GIT_STASH_LIST: &str = "git_stash_list";
pub const GIT_STASH_POP: &str = "git_stash_pop";
pub const GIT_STATUS: &str = "git_status";
pub const GIT_TAG: &str = "git_tag";
pub const GIT_TAG_CREATE: &str = "git_tag_create";
pub const GIT_TAG_LIST: &str = "git_tag_list";
pub const GIT_WORKTREE_ADD: &str = "git_worktree_add";
pub const GIT_WORKTREE_LIST: &str = "git_worktree_list";
pub const GIT_WORKTREE_LOCK: &str = "git_worktree_lock";
pub const GIT_WORKTREE_PRUNE: &str = "git_worktree_prune";
pub const GIT_WORKTREE_REMOVE: &str = "git_worktree_remove";
pub const GIT_WORKTREE_UNLOCK: &str = "git_worktree_unlock";

// GitHub tools
pub const GITHUB_ACCEPT_REPO_INVITATION: &str = "github_accept_repo_invitation";
pub const GITHUB_ADD_ISSUE_COMMENT: &str = "github_add_issue_comment";
pub const GITHUB_ADD_PULL_REQUEST_REVIEW_COMMENT: &str = "github_add_pull_request_review_comment";
pub const GITHUB_CODE_SCANNING_ALERTS: &str = "github_code_scanning_alerts";
pub const GITHUB_CREATE_BRANCH: &str = "github_create_branch";
pub const GITHUB_CREATE_ISSUE: &str = "github_create_issue";
pub const GITHUB_CREATE_OR_UPDATE_FILE: &str = "github_create_or_update_file";
pub const GITHUB_CREATE_PULL_REQUEST: &str = "github_create_pull_request";
pub const GITHUB_CREATE_PULL_REQUEST_REVIEW: &str = "github_create_pull_request_review";
pub const GITHUB_CREATE_RELEASE: &str = "github_create_release";
pub const GITHUB_CREATE_REPOSITORY: &str = "github_create_repository";
pub const GITHUB_DELETE_BRANCH: &str = "github_delete_branch";
pub const GITHUB_DELETE_FILE: &str = "github_delete_file";
pub const GITHUB_FORK_REPOSITORY: &str = "github_fork_repository";
pub const GITHUB_GET_COMMIT: &str = "github_get_commit";
pub const GITHUB_GET_FILE_CONTENTS: &str = "github_get_file_contents";
pub const GITHUB_GET_ISSUE: &str = "github_get_issue";
pub const GITHUB_GET_ISSUE_COMMENTS: &str = "github_get_issue_comments";
pub const GITHUB_GET_ME: &str = "github_get_me";
pub const GITHUB_GET_PULL_REQUEST_FILES: &str = "github_get_pull_request_files";
pub const GITHUB_GET_PULL_REQUEST_REVIEWS: &str = "github_get_pull_request_reviews";
pub const GITHUB_GET_PULL_REQUEST_STATUS: &str = "github_get_pull_request_status";
pub const GITHUB_LIST_BRANCHES: &str = "github_list_branches";
pub const GITHUB_LIST_COMMITS: &str = "github_list_commits";
pub const GITHUB_LIST_ISSUES: &str = "github_list_issues";
pub const GITHUB_LIST_PULL_REQUESTS: &str = "github_list_pull_requests";
pub const GITHUB_LIST_REPOS: &str = "github_list_repos";
pub const GITHUB_MERGE_PULL_REQUEST: &str = "github_merge_pull_request";
pub const GITHUB_PENDING_INVITATIONS: &str = "github_pending_invitations";
pub const GITHUB_PUSH_FILE: &str = "github_push_file";
pub const GITHUB_PUSH_FILES: &str = "github_push_files";
pub const GITHUB_REQUEST_COPILOT_REVIEW: &str = "github_request_copilot_review";
pub const GITHUB_SEARCH_CODE: &str = "github_search_code";
pub const GITHUB_SEARCH_ISSUES: &str = "github_search_issues";
pub const GITHUB_SEARCH_REPOSITORIES: &str = "github_search_repositories";
pub const GITHUB_SEARCH_USERS: &str = "github_search_users";
pub const GITHUB_SECRET_SCANNING_ALERTS: &str = "github_secret_scanning_alerts";
pub const GITHUB_UPDATE_ISSUE: &str = "github_update_issue";
pub const GITHUB_UPDATE_PULL_REQUEST: &str = "github_update_pull_request";

// Browser tools
pub const BROWSER_AGENT: &str = "browser_agent";
pub const BROWSER_AGENT_KILL: &str = "browser_agent_kill";
pub const BROWSER_CLICK: &str = "browser_click";
pub const BROWSER_EVAL: &str = "browser_eval";
pub const BROWSER_EXTRACT_TEXT: &str = "browser_extract_text";
pub const BROWSER_NAVIGATE: &str = "browser_navigate";
pub const BROWSER_RESEARCH: &str = "browser_research";
pub const BROWSER_SCREENSHOT: &str = "browser_screenshot";
pub const BROWSER_SCROLL: &str = "browser_scroll";
pub const BROWSER_TYPE_TEXT: &str = "browser_type_text";
pub const BROWSER_WEB_SEARCH: &str = "browser_web_search";

// Database tools
pub const DB_EXECUTE_SQL: &str = "db_execute_sql";
pub const DB_LIST_SCHEMAS: &str = "db_list_schemas";
pub const DB_LIST_TABLES: &str = "db_list_tables";
pub const DB_POOL_STATS: &str = "db_pool_stats";
pub const DB_STORED_PROCEDURES: &str = "db_stored_procedures";
pub const DB_TABLE_INDEXES: &str = "db_table_indexes";
pub const DB_TABLE_SCHEMA: &str = "db_table_schema";

// Terminal tools
pub const START_TERMINAL: &str = "start_terminal";
pub const TERMINAL: &str = "terminal";

// Process tools
pub const PROCESS_KILL: &str = "process_kill";
pub const PROCESS_LIST: &str = "process_list";

// Introspection tools
pub const INTROSPECTION_GET_EVENTS: &str = "introspection_get_events";
pub const INTROSPECTION_INSPECT_TOOL_CALLS: &str = "introspection_inspect_tool_calls";
pub const INTROSPECTION_INSPECT_USAGE_STATS: &str = "introspection_inspect_usage_stats";
pub const INTROSPECTION_LIST_TOOLS: &str = "introspection_list_tools";
pub const INSPECT_TOOL_CALLS: &str = "inspect_tool_calls";
pub const INSPECT_USAGE_STATS: &str = "inspect_usage_stats";

// Prompt tools
pub const PROMPT_ADD: &str = "prompt_add";
pub const PROMPT_DELETE: &str = "prompt_delete";
pub const PROMPT_EDIT: &str = "prompt_edit";
pub const PROMPT_GET: &str = "prompt_get";

// Config tools
pub const CONFIG_GET: &str = "config_get";
pub const CONFIG_SET: &str = "config_set";

// Citescrape tools
pub const CITESCRAPE_FETCH: &str = "fetch";
pub const CITESCRAPE_SCRAPE_URL: &str = "scrape_url";
pub const CITESCRAPE_WEB_SEARCH: &str = "web_search";
pub const FETCH: &str = "fetch";
pub const SCRAPE_URL: &str = "scrape_url";
pub const WEB_SEARCH: &str = "web_search";

// Reasoning tools
pub const REASONER: &str = "reasoner";
pub const SEQUENTIAL_THINKING: &str = "sequential_thinking";

// ============================================================================
// PORT ASSIGNMENTS
// ============================================================================

/// HTTP port for browser MCP server
pub const PORT_BROWSER: u16 = 30438;

/// HTTP port for citescrape MCP server
pub const PORT_CITESCRAPE: u16 = 30439;

/// HTTP port for claude_agent MCP server
pub const PORT_CLAUDE_AGENT: u16 = 30440;

/// HTTP port for config MCP server
pub const PORT_CONFIG: u16 = 30441;

/// HTTP port for database MCP server
pub const PORT_DATABASE: u16 = 30442;

/// HTTP port for filesystem MCP server
pub const PORT_FILESYSTEM: u16 = 30443;

/// HTTP port for git MCP server
pub const PORT_GIT: u16 = 30444;

/// HTTP port for github MCP server
pub const PORT_GITHUB: u16 = 30445;

/// HTTP port for introspection MCP server
pub const PORT_INTROSPECTION: u16 = 30446;

/// HTTP port for process MCP server
pub const PORT_PROCESS: u16 = 30447;

/// HTTP port for prompt MCP server
pub const PORT_PROMPT: u16 = 30448;

/// HTTP port for reasoner MCP server
pub const PORT_REASONER: u16 = 30449;

/// HTTP port for sequential_thinking MCP server
pub const PORT_SEQUENTIAL_THINKING: u16 = 30450;

/// HTTP port for terminal MCP server
pub const PORT_TERMINAL: u16 = 30451;

/// HTTP port for candle_agent MCP server (also serves memory tools)
pub const PORT_CANDLE_AGENT: u16 = 30452;

/// Minimum port in allocated range
pub const PORT_MIN: u16 = 30438;

/// Maximum port in allocated range
pub const PORT_MAX: u16 = 30452;

// ============================================================================
// CATEGORY-TO-PORT MAPPING
// ============================================================================

/// Static mapping of categories to their assigned HTTP ports
///
/// This is the canonical source of truth for port assignments.
/// Used by:
/// - kodegen routing table (stdio/metadata/routing.rs)
/// - kodegend daemon configuration (kodegend/src/config.rs)
/// - Monitor command (kodegen/src/commands/monitor.rs)
pub const CATEGORY_PORTS: &[(&Category, u16)] = &[
    (CATEGORY_BROWSER, PORT_BROWSER),
    (CATEGORY_CANDLE_AGENT, PORT_CANDLE_AGENT),
    (CATEGORY_CITESCRAPE, PORT_CITESCRAPE),
    (CATEGORY_CLAUDE_AGENT, PORT_CLAUDE_AGENT),
    (CATEGORY_CONFIG, PORT_CONFIG),
    (CATEGORY_DATABASE, PORT_DATABASE),
    (CATEGORY_FILESYSTEM, PORT_FILESYSTEM),
    (CATEGORY_GIT, PORT_GIT),
    (CATEGORY_GITHUB, PORT_GITHUB),
    (CATEGORY_INTROSPECTION, PORT_INTROSPECTION),
    (CATEGORY_PROCESS, PORT_PROCESS),
    (CATEGORY_PROMPT, PORT_PROMPT),
    (CATEGORY_REASONER, PORT_REASONER),
    (CATEGORY_SEQUENTIAL_THINKING, PORT_SEQUENTIAL_THINKING),
    (CATEGORY_TERMINAL, PORT_TERMINAL),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_uniqueness() {
        let mut ports: Vec<u16> = CATEGORY_PORTS.iter().map(|(_, p)| *p).collect();
        let original_len = ports.len();
        ports.sort_unstable();
        ports.dedup();
        assert_eq!(
            ports.len(),
            original_len,
            "Duplicate port assignments found"
        );
    }

    #[test]
    fn test_port_range() {
        for (cat, port) in CATEGORY_PORTS {
            assert!(
                *port >= PORT_MIN && *port <= PORT_MAX,
                "Port {} for category {} outside valid range {}-{}",
                port,
                cat.name,
                PORT_MIN,
                PORT_MAX
            );
        }
    }

    #[test]
    fn test_category_count() {
        assert_eq!(
            CATEGORY_PORTS.len(),
            14,
            "Expected 14 category-port mappings (memory uses candle_agent port)"
        );
    }
}
