use tauri::{command, State};
use crate::AppState;
use crate::search::provider::SearchItem;

#[command]
pub async fn search(
    state: State<'_, AppState>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchItem>, String> {
    let engine = state.search_engine.lock();
    let results = engine.search(&query, limit.unwrap_or(20));
    Ok(results)
}

#[command]
pub async fn get_recent(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<SearchItem>, String> {
    let engine = state.search_engine.lock();
    let mut items = engine.search("", limit.unwrap_or(10));
    // Sort by use count for recent items
    items.sort_by(|a, b| b.use_count.cmp(&a.use_count));
    Ok(items)
}
