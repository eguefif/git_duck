use git2::{Repository, RepositoryState};
use std::fmt;

type ErrorType = Box<dyn std::error::Error>;

#[derive(Debug)]
enum DuckError {
    HeadName,
}

impl fmt::Display for DuckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DuckError::HeadName => write!(f, "Error:Impossible to get head name"),
        }
    }
}

impl std::error::Error for DuckError {}

fn get_head_name(repo: String) -> Result<String, ErrorType> {
    let repo = Repository::open(repo)?;
    let head = repo.head()?;
    let name = head.name().ok_or_else(|| DuckError::HeadName)?;
    Ok(name.to_string())
}

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

fn get_state_string(state: RepositoryState) -> String {
    match state {
        RepositoryState::Clean => "clean".to_string(),
        RepositoryState::Merge => "merge".to_string(),
        RepositoryState::Revert => "revert".to_string(),
        RepositoryState::RevertSequence => "revert sequence".to_string(),
        RepositoryState::CherryPick => "cherry pick".to_string(),
        RepositoryState::CherryPickSequence => "cherry pick sequence".to_string(),
        RepositoryState::Bisect => "bisect".to_string(),
        RepositoryState::Rebase => "rebase".to_string(),
        RepositoryState::RebaseInteractive => "rebase interactive".to_string(),
        RepositoryState::RebaseMerge => "rebase merge".to_string(),
        RepositoryState::ApplyMailbox => "apply mail box".to_string(),
        RepositoryState::ApplyMailboxOrRebase => "apply mail box or rebase".to_string(),
    }
}

#[tauri::command]
fn get_repo_state(repo: String) -> Result<String, String> {
    match Repository::open(repo) {
        Ok(repo) => Ok(get_state_string(repo.state())),
        Err(e) => Err(format!("{}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![list_commits_head, get_repo_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
