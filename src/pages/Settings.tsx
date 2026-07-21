import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app";
import { openUrl } from "@tauri-apps/plugin-opener";
import { open } from "@tauri-apps/plugin-dialog";
import { Save, Folder, RefreshCw, Download } from "lucide-react";

export default function Settings() {
  const [backupDir, setBackupDir] = useState("");
  const [saving, setSaving] = useState(false);
  const [currentVersion, setCurrentVersion] = useState("Unknown");
  const [updateVersion, setUpdateVersion] = useState<string | null>(null);
  const [checkingUpdate, setCheckingUpdate] = useState(false);

  useEffect(() => {
    async function loadSettings() {
      try {
        const dir: string = await invoke("get_backup_dir");
        setBackupDir(dir);
        
        try {
          const ver = await getVersion();
          setCurrentVersion(ver);
        } catch (e) {
          console.error("Failed to get version", e);
        }
      } catch (err) {
        console.error("Failed to load settings:", err);
      }
    }
    loadSettings();
  }, []);

  const handleSave = async () => {
    setSaving(true);
    try {
      await invoke("set_backup_dir", { dir: backupDir });
      alert("Settings saved!");
    } catch (err) {
      alert(`Failed to save settings: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  const handleSelectFolder = async () => {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Select Backup Folder"
      });
      if (selectedPath) {
        setBackupDir(selectedPath as string);
      }
    } catch (err) {
      console.error("Failed to select folder:", err);
    }
  };

  const handleCheckUpdate = async () => {
    setCheckingUpdate(true);
    setUpdateVersion(null);
    try {
      const latest: string | null = await invoke("check_launcher_update");
      if (latest) {
        setUpdateVersion(latest);
      } else {
        alert("You are on the latest version!");
      }
    } catch (err) {
      alert(`Failed to check for updates: ${err}`);
    } finally {
      setCheckingUpdate(false);
    }
  };

  return (
    <>
      <div className="content-header">
        <h1>Settings</h1>
        <p style={{ color: "var(--text-secondary)", marginTop: "8px" }}>
          Configure your launcher preferences.
        </p>
      </div>
      
      <div className="content-body" style={{ maxWidth: "600px" }}>
        <div className="glass-panel" style={{ padding: "24px" }}>
          <h2 style={{ fontSize: "1.125rem", marginBottom: "16px" }}>Data Management</h2>
          
          <div className="input-group">
            <label>Master Backup Directory</label>
            <div style={{ display: "flex", gap: "8px" }}>
              <input 
                type="text" 
                className="input-field" 
                value={backupDir}
                onChange={(e) => setBackupDir(e.target.value)}
                placeholder="/Users/username/Dropbox/WebAppBackups"
                style={{ flex: 1 }}
              />
              <button className="btn btn-secondary" onClick={handleSelectFolder} title="Browse Folders">
                <Folder size={16} />
              </button>
            </div>
            <span style={{ fontSize: "0.75rem", color: "var(--text-tertiary)", marginTop: "4px", display: "block" }}>
              When you click 'Backup' on an app, its data folder will be zipped and copied here.
            </span>
          </div>
          
          <div style={{ marginTop: "24px", display: "flex", justifyContent: "flex-end" }}>
            <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
              <Save size={16} /> {saving ? "Saving..." : "Save Settings"}
            </button>
          </div>
        </div>

        <div className="glass-panel" style={{ padding: "24px", marginTop: "24px" }}>
          <h2 style={{ fontSize: "1.125rem", marginBottom: "16px" }}>Launcher Updates</h2>
          
          <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between" }}>
            <div>
              <p style={{ fontWeight: "500" }}>Current Version: <span className="badge badge-prod">v{currentVersion}</span></p>
              {updateVersion && (
                <p style={{ color: "var(--accent-success)", marginTop: "8px", fontWeight: "500" }}>
                  🎉 Update v{updateVersion} is available!
                </p>
              )}
            </div>
            
            <div>
              {updateVersion ? (
                <button 
                  className="btn btn-primary" 
                  onClick={() => openUrl("https://github.com/LucasFenaux/launcher/releases/latest")}
                >
                  <Download size={16} /> Download Update
                </button>
              ) : (
                <button className="btn btn-secondary" onClick={handleCheckUpdate} disabled={checkingUpdate}>
                  <RefreshCw size={16} className={checkingUpdate ? "animate-spin" : ""} /> 
                  {checkingUpdate ? "Checking..." : "Check for Updates"}
                </button>
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
