use nucleo::{Matcher, Utf32Str};
use crate::search::provider::SearchItem;

/// High-performance fuzzy search engine powered by nucleo (same algorithm as fzf).
pub struct SearchEngine {
    items: Vec<SearchItem>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Index searchable items
    pub fn index(&mut self, items: Vec<SearchItem>) {
        self.items = items;
    }

    /// Add a single item to the index
    pub fn add(&mut self, item: SearchItem) {
        self.items.push(item);
    }

    /// Perform fuzzy search and return ranked results.
    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchItem> {
        if query.is_empty() {
            return self.items.iter().take(limit).cloned().collect();
        }

        let mut matcher = Matcher::new(nucleo::Config::DEFAULT);
        let mut buf = Vec::new();
        let query_str = Utf32Str::new(query, &mut buf);

        let mut scored: Vec<(f64, usize)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                let mut buf = Vec::new();
                let haystack_str = Utf32Str::new(&item.search_text, &mut buf);
                matcher
                    .fuzzy_match(haystack_str, query_str)
                    .map(|score| (score as f64, i))
            })
            .collect();

        // Sort by score descending
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        scored
            .into_iter()
            .take(limit)
            .map(|(_, i)| self.items[i].clone())
            .collect()
    }
}
