use std::path::PathBuf;

/// Placeholder for file indexing system.
/// Phase 3 will implement SQLite FTS5-backed file indexer.
pub struct FileIndexer {
    indexed_paths: Vec<PathBuf>,
    exclude_patterns: Vec<String>,
}

impl FileIndexer {
    pub fn new(paths: Vec<PathBuf>, exclude: Vec<String>) -> Self {
        Self {
            indexed_paths: paths,
            exclude_patterns: exclude,
        }
    }

    /// Perform full index scan (to be implemented in Phase 3)
    #[allow(dead_code)]
    pub fn full_scan(&self) {
        log::info!("File indexer: full scan starting...");
        // Phase 3: Walk indexed_paths, filter by exclude_patterns,
        // index into SQLite FTS5
    }

    /// Set up filesystem watcher for incremental updates
    #[allow(dead_code)]
    pub fn watch(&self) {
        log::info!("File indexer: filesystem watcher starting...");
        // Phase 3: Use notify crate to watch indexed directories
        // for changes and update the index incrementally
    }
}
