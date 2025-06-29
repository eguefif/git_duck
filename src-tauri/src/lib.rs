use git2::Repository;

#[tauri::command]
fn list_commits_head(repo: String) -> Vec<(String, String)> {
    let repo = Repository::open(repo).unwrap();
    let head = repo.head().unwrap();
    let head_name = head.name().unwrap();
    let log = repo.reflog(head_name).unwrap();
    log.iter().rfold(Vec::new(), |mut acc, entry| {
        let message = entry.message().unwrap_or("No commit message").to_string();
        acc.push((entry.id_new().to_string(), message));
        acc
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_commits_head])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
