/// File search via Everything's es.exe CLI.
/// Everything must be installed and running.
use crate::search::provider::{ItemAction, ItemType, SearchItem};

/// Search files using Everything es.exe.
pub fn search_everything(query: &str, max_results: usize) -> Vec<SearchItem> {
    if query.is_empty() {
        return Vec::new();
    }

    // Find es.exe: check next to binary, then Everything dir, then PATH
    let es_paths = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("es.exe"))),
        Some(std::path::PathBuf::from(
            "C:\\Program Files\\Everything\\es.exe",
        )),
        Some(std::path::PathBuf::from(
            "D:\\Program Files\\Everything\\es.exe",
        )),
    ];

    let es_path = es_paths
        .iter()
        .flatten()
        .find(|p| p.exists());

    let es_path = match es_path {
        Some(p) => p,
        None => {
            log::warn!("es.exe not found — install Everything with CLI tools");
            return Vec::new();
        }
    };

    let output = std::process::Command::new(es_path)
        .args(["-n", &max_results.to_string(), query])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            parse_es_output(&text)
        }
        Ok(out) => {
            let err = String::from_utf8_lossy(&out.stderr);
            log::warn!("es.exe error: {}", err.trim());
            Vec::new()
        }
        Err(e) => {
            log::error!("Failed to run es.exe: {}", e);
            Vec::new()
        }
    }
}

fn parse_es_output(text: &str) -> Vec<SearchItem> {
    let mut items = Vec::new();
    for line in text.lines().take(100) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let path = line.to_string();
        let is_dir = path.ends_with('\\') || path.ends_with('/');
        let clean = path.trim_end_matches('\\').trim_end_matches('/').to_string();

        let name = std::path::Path::new(&clean)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| clean.clone());

        let folder = std::path::Path::new(&clean)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        items.push(SearchItem {
            id: format!("file:{}", clean),
            title: name,
            subtitle: folder,
            icon: if is_dir {
                "📁".to_string()
            } else {
                "📄".to_string()
            },
            item_type: if is_dir {
                ItemType::Folder
            } else {
                ItemType::File
            },
            search_text: clean.clone(),
            action: if is_dir {
                ItemAction::OpenFolder { path: clean }
            } else {
                ItemAction::OpenFile { path: clean }
            },
            use_count: 0,
        });
    }
    items
}

pub mod commands {
    use tauri::command;

    #[command]
    pub async fn search_files(query: String, limit: Option<usize>) -> Result<Vec<crate::search::provider::SearchItem>, String> {
        Ok(super::search_everything(&query, limit.unwrap_or(20)))
    }
}
