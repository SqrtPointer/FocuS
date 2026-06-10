use tauri::{command, State};
use crate::AppState;
use crate::search::provider::SearchItem;

#[command]
pub async fn search(
    state: State<'_, AppState>,
    query: String,
    limit: Option<usize>,
) -> Result<Vec<SearchItem>, String> {
    let limit = limit.unwrap_or(20);

    if query.is_empty() {
        let engine = state.search_engine.lock();
        return Ok(engine.search("", limit));
    }

    // Run app search + file search in parallel
    let half = (limit / 2).max(5);
    let engine = state.search_engine.lock();
    let mut results = engine.search(&query, half);

    // Always try file search for non-empty queries
    let file_results = crate::scanner::files::search_everything(&query, limit - results.len());
    results.extend(file_results);

    Ok(results)
}

#[command]
pub async fn get_recent(
    state: State<'_, AppState>,
    limit: Option<usize>,
) -> Result<Vec<SearchItem>, String> {
    let engine = state.search_engine.lock();
    let mut items = engine.search("", limit.unwrap_or(10));
    items.sort_by(|a, b| b.use_count.cmp(&a.use_count));
    Ok(items)
}
