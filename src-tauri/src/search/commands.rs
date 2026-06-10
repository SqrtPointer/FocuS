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
    let engine = state.search_engine.lock();

    // Search indexed items (applications)
    let mut results = engine.search(&query, limit);

    // Also search files via Everything
    if results.len() < limit {
        let file_results = crate::scanner::files::search_everything(&query, limit - results.len());
        results.extend(file_results);
    }

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
