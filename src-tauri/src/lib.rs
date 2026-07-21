use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppProcess {
    pid: u32,
    port: u16,
}

struct AppState {
    processes: Mutex<HashMap<String, AppProcess>>,
}

fn get_state_file() -> PathBuf {
    let mut path = get_library_dir();
    path.push("processes.json");
    path
}

fn save_state(processes: &HashMap<String, AppProcess>) {
    let state_file = get_state_file();
    if let Ok(content) = serde_json::to_string_pretty(processes) {
        let _ = fs::write(state_file, content);
    }
}

fn load_state() -> HashMap<String, AppProcess> {
    let state_file = get_state_file();
    if let Ok(content) = fs::read_to_string(state_file) {
        if let Ok(processes) = serde_json::from_str(&content) {
            return processes;
        }
    }
    HashMap::new()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppInfo {
    id: String,
    name: String,
    description: String,
    repo_url: String,
    port: u16,
    published_port: Option<u16>,
    mode: String,
    is_installed: bool,
    is_running: bool,
    version: Option<String>,
}

#[derive(Serialize)]
pub struct DepsCheck {
    ok: bool,
    message: String,
}

fn get_library_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    path.push("Library");
    path.push("Application Support");
    path.push("WebAppLauncher");
    path.push("Apps");
    path
}

fn get_app_dir(app: &AppInfo) -> PathBuf {
    if app.mode == "dev" {
        PathBuf::from(&app.repo_url)
    } else {
        let mut p = get_library_dir();
        p.push(&app.id);
        p
    }
}

fn kill_process_tree(pid: sysinfo::Pid, sys: &sysinfo::System) {
    let mut pids_to_kill = vec![pid];
    let mut i = 0;
    while i < pids_to_kill.len() {
        let current_pid = pids_to_kill[i];
        for (child_pid, process) in sys.processes() {
            if let Some(parent) = process.parent() {
                if parent == current_pid {
                    pids_to_kill.push(*child_pid);
                }
            }
        }
        i += 1;
    }
    
    // Kill in reverse order (children first)
    for p in pids_to_kill.into_iter().rev() {
        if let Some(process) = sys.process(p) {
            process.kill();
        }
    }
}

#[tauri::command]
fn check_dependencies() -> DepsCheck {
    DepsCheck { ok: true, message: "All dependencies met.".into() }
}

fn get_catalog_base() -> Vec<AppInfo> {
    let mut apps = vec![
        AppInfo {
            id: "application-tracker".into(),
            name: "Job Application Tracker".into(),
            description: "Track your job applications locally.".into(),
            repo_url: "https://github.com/LucasFenaux/job-application-tracker".into(),
            port: 3000,
            published_port: None,
            mode: "prod".into(),
            is_installed: false,
            is_running: false,
            version: None,
        },
        AppInfo {
            id: "literature-review-graph".into(),
            name: "Literature Map".into(),
            description: "A tool for literature review and graphing.".into(),
            repo_url: "https://github.com/LucasFenaux/literature-review-graph".into(),
            port: 3000,
            published_port: None,
            mode: "prod".into(),
            is_installed: false,
            is_running: false,
            version: None,
        },
        AppInfo {
            id: "portfolio-stats".into(),
            name: "Portfolio Analytics".into(),
            description: "Analytics for your portfolio.".into(),
            repo_url: "https://github.com/LucasFenaux/portfolio-analytics".into(),
            port: 5173,
            published_port: None,
            mode: "prod".into(),
            is_installed: false,
            is_running: false,
            version: None,
        },
        AppInfo {
            id: "productivity-app".into(),
            name: "Local Core Planner".into(),
            description: "All-in-one productivity suite.".into(),
            repo_url: "https://github.com/LucasFenaux/aio-productivity-app".into(),
            port: 5173,
            published_port: None,
            mode: "prod".into(),
            is_installed: false,
            is_running: false,
            version: None,
        },
    ];

    #[cfg(debug_assertions)]
    {
        let mut dev_apps_path = get_library_dir();
        dev_apps_path.push("dev_apps.json");
        
        if dev_apps_path.exists() {
            if let Ok(content) = fs::read_to_string(dev_apps_path) {
                if let Ok(dev_apps) = serde_json::from_str::<Vec<AppInfo>>(&content) {
                    apps.extend(dev_apps);
                }
            }
        }
    }
    
    apps
}

#[tauri::command]
fn get_catalog(state: State<'_, AppState>) -> Vec<AppInfo> {
    let mut apps = get_catalog_base();
    
    let sys = sysinfo::System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::everything())
    );
    
    let mut processes = state.processes.lock().unwrap();
    let mut dead_apps = vec![];
    
    for app in apps.iter_mut() {
        let app_dir = get_app_dir(app);
        
        if app.mode == "prod" {
            let binary_name = if std::env::consts::OS == "windows" { format!("{}.exe", app.id) } else { format!("app-{}", std::env::consts::OS) };
            let mut binary_path = app_dir.clone();
            binary_path.push(binary_name);
            app.is_installed = binary_path.exists();
            
            if app.is_installed {
                let mut version_path = app_dir.clone();
                version_path.push(".version");
                if let Ok(ver) = fs::read_to_string(version_path) {
                    app.version = Some(ver);
                }
            }
        } else {
            app.is_installed = true;
        }
        
        if let Some(proc) = processes.get(&app.id) {
            if sys.process(sysinfo::Pid::from_u32(proc.pid)).is_some() {
                app.is_running = true;
                app.published_port = Some(proc.port);
            } else {
                dead_apps.push(app.id.clone());
            }
        }
    }
    
    if !dead_apps.is_empty() {
        for id in dead_apps {
            processes.remove(&id);
        }
        save_state(&processes);
    }
    
    apps
}

#[tauri::command]
fn install_app(app_id: String) -> Result<(), String> {
    let apps = get_catalog_base();
    let app = apps.iter().find(|a| a.id == app_id).ok_or("App not found")?;
    
    if app.mode == "dev" {
        return Ok(());
    }
    
    let app_dir = get_app_dir(app);
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    
    let parts: Vec<&str> = app.repo_url.split('/').collect();
    if parts.len() < 2 { return Err("Invalid repo url".into()); }
    let repo = parts.last().unwrap();
    let owner = parts[parts.len() - 2];
    
    let api_url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);
    let client = reqwest::blocking::Client::builder()
        .user_agent("WebAppLauncher")
        .build().unwrap();
        
    let resp: serde_json::Value = client.get(&api_url).send().map_err(|e| e.to_string())?.json().map_err(|e| e.to_string())?;
    
    let tag = resp.get("tag_name").and_then(|t| t.as_str()).unwrap_or("unknown");
    let assets = resp.get("assets").and_then(|a| a.as_array()).ok_or("No release assets found")?;
    
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;
    
    let mut download_url = None;
    for asset in assets {
        let name = asset.get("name").and_then(|n| n.as_str()).unwrap_or("").to_lowercase();
        if target_os == "macos" {
            if name.contains("macos") && (
                (target_arch == "aarch64" && name.contains("arm64")) || 
                (target_arch == "x86_64" && (name.contains("x64") || name.contains("x86_64"))) || 
                (!name.contains("arm") && !name.contains("x64") && !name.contains("x86_64") && !name.contains("arm64")) 
            ) {
                download_url = asset.get("browser_download_url").and_then(|u| u.as_str());
                break;
            }
        } else if target_os == "windows" {
            if name.contains("windows") || name.contains("win") {
                download_url = asset.get("browser_download_url").and_then(|u| u.as_str());
                break;
            }
        } else {
            if name.contains("linux") {
                download_url = asset.get("browser_download_url").and_then(|u| u.as_str());
                break;
            }
        }
    }
    
    if let Some(url) = download_url {
        let binary_resp = client.get(url).send().map_err(|e| e.to_string())?.bytes().map_err(|e| e.to_string())?;
        
        let binary_name = if target_os == "windows" { format!("{}.exe", app.id) } else { format!("app-{}", std::env::consts::OS) };
        let mut binary_path = app_dir.clone();
        binary_path.push(&binary_name);
        
        fs::write(&binary_path, binary_resp).map_err(|e| e.to_string())?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&binary_path).map_err(|e| e.to_string())?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&binary_path, perms).map_err(|e| e.to_string())?;
        }
        
        let mut version_path = app_dir.clone();
        version_path.push(".version");
        fs::write(version_path, tag).map_err(|e| e.to_string())?;
        
        let mut data_dir = app_dir.clone();
        data_dir.push("data");
        let _ = fs::create_dir_all(data_dir);
    } else {
        return Err("No compatible binary found for this OS/Arch on GitHub Releases.".into());
    }
    
    Ok(())
}

#[tauri::command]
fn launch_app(app_id: String, state: State<'_, AppState>) -> Result<Option<u16>, String> {
    let apps = get_catalog_base();
    let app = apps.iter().find(|a| a.id == app_id).ok_or("App not found")?;
    let app_dir = get_app_dir(app);
    
    let port = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?.local_addr().unwrap().port();
    
    if app.mode == "dev" {
        let backend_port = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?.local_addr().unwrap().port();
        let mut start_script = app_dir.clone();
        start_script.push("start.sh");
        if start_script.exists() {
            let child = Command::new("bash")
                .arg("start.sh")
                .env("PORT", port.to_string())
                .env("BACKEND_PORT", backend_port.to_string())
                .current_dir(&app_dir)
                .spawn()
                .map_err(|e| e.to_string())?;
                
            let mut procs = state.processes.lock().unwrap();
            procs.insert(app_id, AppProcess {
                pid: child.id(),
                port,
            });
            save_state(&procs);
            
            return Ok(Some(port));
        } else {
            return Err("No start.sh found for dev app. Create one to run your dev server!".into());
        }
    } else {
        let binary_name = if std::env::consts::OS == "windows" { format!("{}.exe", app.id) } else { format!("app-{}", std::env::consts::OS) };
        let mut binary_path = app_dir.clone();
        binary_path.push(&binary_name);
        
        if !binary_path.exists() {
            return Err("Binary not found. Please install the app first.".into());
        }
        
        let child = Command::new(&binary_path)
            .env("PORT", port.to_string())
            .current_dir(&app_dir)
            .spawn()
            .map_err(|e| e.to_string())?;
            
        let mut procs = state.processes.lock().unwrap();
        procs.insert(app_id, AppProcess {
            pid: child.id(),
            port,
        });
        save_state(&procs);
        
        return Ok(Some(port));
    }
}

#[tauri::command]
fn stop_app(app_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut processes = state.processes.lock().unwrap();
    if let Some(proc) = processes.remove(&app_id) {
        let sys = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::everything())
        );
        kill_process_tree(sysinfo::Pid::from_u32(proc.pid), &sys);
        save_state(&processes);
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Default)]
struct Settings {
    backup_dir: Option<String>,
}

fn get_settings_path() -> PathBuf {
    let mut path = get_library_dir();
    path.push("settings.json");
    path
}

#[tauri::command]
fn get_backup_dir() -> String {
    let settings_path = get_settings_path();
    if settings_path.exists() {
        if let Ok(content) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                if let Some(dir) = settings.backup_dir {
                    return dir;
                }
            }
        }
    }
    let mut default_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    default_path.push("WebAppBackups");
    default_path.to_string_lossy().to_string()
}

#[tauri::command]
fn set_backup_dir(dir: String) -> Result<(), String> {
    let settings_path = get_settings_path();
    let mut settings = Settings::default();
    
    if settings_path.exists() {
        if let Ok(content) = fs::read_to_string(&settings_path) {
            if let Ok(parsed) = serde_json::from_str::<Settings>(&content) {
                settings = parsed;
            }
        }
    }
    
    settings.backup_dir = Some(dir);
    
    let content = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&settings_path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn backup_data(app_id: String) -> Result<(), String> {
    let apps = get_catalog_base();
    let app = apps.iter().find(|a| a.id == app_id).ok_or("App not found")?;
    let app_dir = get_app_dir(app);
    
    let mut data_dir = app_dir.clone();
    data_dir.push("data");
    
    if !data_dir.exists() {
        return Err("No data folder found to backup".into());
    }
    
    let backup_dest = get_backup_dir();
    fs::create_dir_all(&backup_dest).map_err(|e| e.to_string())?;
    
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    
    let mut zip_path = PathBuf::from(&backup_dest);
    zip_path.push(format!("{}_{}.zip", app.id, timestamp));
    
    let status = Command::new("zip")
        .arg("-r")
        .arg(&zip_path)
        .arg("data")
        .current_dir(data_dir.parent().unwrap())
        .status()
        .map_err(|e| e.to_string())?;
        
    if !status.success() {
        return Err("Failed to create backup zip".into());
    }
    
    Ok(())
}

#[tauri::command]
fn check_update(app_id: String) -> Result<bool, String> {
    let apps = get_catalog_base();
    let app = apps.iter().find(|a| a.id == app_id).ok_or("App not found")?;
    
    if app.mode != "prod" {
        return Ok(false);
    }
    
    let app_dir = get_app_dir(app);
    let mut version_path = app_dir.clone();
    version_path.push(".version");
    
    if !version_path.exists() {
        return Ok(false);
    }
    
    let current_version = fs::read_to_string(version_path).unwrap_or_else(|_| "".into());
    
    let parts: Vec<&str> = app.repo_url.split('/').collect();
    if parts.len() < 2 { return Ok(false); }
    let repo = parts.last().unwrap();
    let owner = parts[parts.len() - 2];
    
    let api_url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);
    let client = reqwest::blocking::Client::builder()
        .user_agent("WebAppLauncher")
        .build().unwrap();
        
    if let Ok(resp) = client.get(&api_url).send() {
        if let Ok(json) = resp.json::<serde_json::Value>() {
            let latest_version = json.get("tag_name").and_then(|t| t.as_str()).unwrap_or("");
            return Ok(latest_version != "" && latest_version != current_version);
        }
    }
    
    Ok(false)
}

#[tauri::command]
fn check_launcher_update(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let current_version = app_handle.package_info().version.to_string();
    
    let api_url = "https://api.github.com/repos/LucasFenaux/launcher/releases/latest";
    let client = reqwest::blocking::Client::builder()
        .user_agent("WebAppLauncher")
        .build().unwrap();
        
    if let Ok(resp) = client.get(api_url).send() {
        if let Ok(json) = resp.json::<serde_json::Value>() {
            let latest_version = json.get("tag_name").and_then(|t| t.as_str()).unwrap_or("");
            let latest_version_clean = latest_version.trim_start_matches('v');
            if latest_version_clean != "" && latest_version_clean != current_version {
                return Ok(Some(latest_version.to_string()));
            }
        }
    }
    
    Ok(None)
}

fn start_backup_watcher() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(10));
            
            let apps = get_catalog_base();
            
            for app in apps {
                let app_dir = get_app_dir(&app);
                
                // Source: the backup folder the app itself manages
                let mut src_dir = app_dir.clone();
                src_dir.push("data");
                src_dir.push("backups");
                
                if !src_dir.exists() || !src_dir.is_dir() {
                    continue;
                }
                
                // Destination: <global_backup_dir>/<app_id>/
                let mut dest_dir = PathBuf::from(get_backup_dir());
                dest_dir.push(&app.id);
                let _ = fs::create_dir_all(&dest_dir);
                
                // Build set of filenames currently in source
                let src_files: std::collections::HashSet<std::ffi::OsString> =
                    fs::read_dir(&src_dir)
                        .map(|entries| {
                            entries.flatten()
                                .filter(|e| e.path().is_file())
                                .map(|e| e.file_name())
                                .collect()
                        })
                        .unwrap_or_default();
                
                // Copy files present in source but missing from dest
                for filename in &src_files {
                    let dest_file = dest_dir.join(filename);
                    if !dest_file.exists() {
                        let _ = fs::copy(src_dir.join(filename), &dest_file);
                    }
                }
                
                // Delete files in dest that no longer exist in source
                if let Ok(dest_entries) = fs::read_dir(&dest_dir) {
                    for entry in dest_entries.flatten() {
                        if entry.path().is_file() && !src_files.contains(&entry.file_name()) {
                            let _ = fs::remove_file(entry.path());
                        }
                    }
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    start_backup_watcher();

    tauri::Builder::default()
        .manage(AppState {
            processes: Mutex::new(load_state()),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            check_dependencies,
            get_catalog,
            install_app,
            launch_app,
            stop_app,
            get_backup_dir,
            set_backup_dir,
            backup_data,
            check_update,
            check_launcher_update
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
