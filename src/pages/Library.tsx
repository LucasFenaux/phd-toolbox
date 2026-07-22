import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { Play, Square, FolderDown, Loader2, ExternalLink, RefreshCw, CloudDownload, RefreshCcw } from "lucide-react";
import { AppInfo } from "../types";

export default function Library() {
  const [apps, setApps] = useState<AppInfo[]>([]);
  const [loadingApp, setLoadingApp] = useState<Record<string, boolean>>({});
  const [updatesAvailable, setUpdatesAvailable] = useState<Record<string, string>>({});
  const [checkingUpdate, setCheckingUpdate] = useState<Record<string, boolean>>({});
  const [isCheckingAllUpdates, setIsCheckingAllUpdates] = useState(false);
  const [draggedAppId, setDraggedAppId] = useState<string | null>(null);
  const [dragOverAppId, setDragOverAppId] = useState<string | null>(null);
  
  const hasCheckedUpdates = useRef(false);

  const fetchApps = async () => {
    try {
      const allApps: AppInfo[] = await invoke("get_catalog");
      let installedApps = allApps.filter(app => app.is_installed);
      
      const savedOrderStr = localStorage.getItem("libraryAppOrder");
      if (savedOrderStr) {
        try {
          const savedOrder: string[] = JSON.parse(savedOrderStr);
          installedApps.sort((a, b) => {
             const idxA = savedOrder.indexOf(a.id);
             const idxB = savedOrder.indexOf(b.id);
             if (idxA !== -1 && idxB !== -1) return idxA - idxB;
             if (idxA !== -1) return -1;
             if (idxB !== -1) return 1;
             return 0;
          });
        } catch (e) {
          console.error("Failed to parse library order", e);
        }
      }
      setApps(installedApps);
    } catch (err) {
      console.error("Failed to fetch library apps:", err);
    }
  };

  const handleDragStart = (e: React.DragEvent, appId: string) => {
    setDraggedAppId(appId);
    e.dataTransfer.effectAllowed = "move";
  };

  const handleDragEnter = (e: React.DragEvent, appId: string) => {
    e.preventDefault();
    setDragOverAppId(appId);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  };

  const handleDragEnd = (e: React.DragEvent) => {
    e.preventDefault();
    
    if (draggedAppId && dragOverAppId && draggedAppId !== dragOverAppId) {
      setApps(currentApps => {
        const newApps = [...currentApps];
        const draggedIndex = newApps.findIndex(a => a.id === draggedAppId);
        const targetIndex = newApps.findIndex(a => a.id === dragOverAppId);

        if (draggedIndex !== -1 && targetIndex !== -1) {
          const [draggedApp] = newApps.splice(draggedIndex, 1);
          newApps.splice(targetIndex, 0, draggedApp);
          localStorage.setItem("libraryAppOrder", JSON.stringify(newApps.map(a => a.id)));
        }
        return newApps;
      });
    }
    
    setDraggedAppId(null);
    setDragOverAppId(null);
  };

  useEffect(() => {
    fetchApps();
    // Poll for status every 3 seconds
    const interval = setInterval(fetchApps, 3000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (apps.length > 0 && !hasCheckedUpdates.current) {
      hasCheckedUpdates.current = true;
      apps.forEach(app => {
        if (app.mode === "prod") {
          invoke("check_update", { appId: app.id }).then(latestVersion => {
            if (latestVersion) {
              setUpdatesAvailable(prev => ({ ...prev, [app.id]: latestVersion as string }));
            }
          }).catch(console.error);
        }
      });
    }
  }, [apps]);

  const handleLaunch = async (appId: string) => {
    setLoadingApp({ ...loadingApp, [appId]: true });
    try {
      const port: number | null = await invoke("launch_app", { appId });
      
      let targetPort = port;
      if (!targetPort) {
        const app = apps.find(a => a.id === appId);
        targetPort = app?.published_port || app?.port || null;
      }
      
      if (targetPort) {
        const url = `http://localhost:${targetPort}`;
        
        // Wait for the server to be reachable (up to 20 seconds)
        for (let i = 0; i < 20; i++) {
          try {
            await fetch(url, { method: 'GET', mode: 'no-cors' });
            break; // Server is up!
          } catch (e) {
            // Not up yet, wait 1 second
            await new Promise(r => setTimeout(r, 1000));
          }
        }
        
        await openUrl(url);
      }
      
      fetchApps();
    } catch (err) {
      console.error("Failed to launch app:", err);
      alert(`Launch failed: ${err}`);
    } finally {
      setLoadingApp({ ...loadingApp, [appId]: false });
    }
  };

  const handleStop = async (appId: string) => {
    setLoadingApp({ ...loadingApp, [appId]: true });
    try {
      await invoke("stop_app", { appId });
      fetchApps();
    } catch (err) {
      console.error("Failed to stop app:", err);
      alert(`Stop failed: ${err}`);
    } finally {
      setLoadingApp({ ...loadingApp, [appId]: false });
    }
  };
  
  const handleBackup = async (appId: string) => {
    try {
      await invoke("backup_data", { appId });
      alert("Backup completed successfully!");
    } catch (err) {
      alert(`Backup failed: ${err}`);
    }
  };

  const handleCheckAllUpdates = async () => {
    if (isCheckingAllUpdates) return;
    setIsCheckingAllUpdates(true);
    
    let updatesFound = 0;
    const newUpdates: Record<string, string> = {};
    
    try {
      // Check for launcher update first
      const launcherUpdate: string | null = await invoke("check_launcher_update");
      if (launcherUpdate) {
        if (window.confirm(`A new version of the Launcher (${launcherUpdate}) is available!\n\nWould you like to open GitHub to download it?`)) {
          await openUrl("https://github.com/LucasFenaux/phd-toolbox/releases/latest");
        }
      }
      
      const prodApps = apps.filter(a => a.mode === "prod");
      for (const app of prodApps) {
        const latestVersion: string | null = await invoke("check_update", { appId: app.id });
        if (latestVersion) {
          newUpdates[app.id] = latestVersion;
          updatesFound++;
        }
      }
      setUpdatesAvailable(prev => ({ ...prev, ...newUpdates }));
      
      if (updatesFound > 0) {
        alert(`Found updates for ${updatesFound} app(s)!`);
      } else {
        alert("All apps are up-to-date!");
      }
    } catch (err) {
      console.error("Failed to check for updates:", err);
      alert(`Check failed: ${err}`);
    } finally {
      setIsCheckingAllUpdates(false);
    }
  };

  const handleUpdate = async (appId: string) => {
    setCheckingUpdate({ ...checkingUpdate, [appId]: true });
    try {
      await invoke("install_app", { appId });
      setUpdatesAvailable(prev => {
        const next = { ...prev };
        delete next[appId];
        return next;
      });
      alert("App updated successfully!");
    } catch (err) {
      console.error("Failed to update app:", err);
      alert(`Update failed: ${err}`);
    } finally {
      setCheckingUpdate({ ...checkingUpdate, [appId]: false });
    }
  };

  return (
    <>
      <div className="content-header" style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <div>
          <h1>Library</h1>
          <p style={{ color: "var(--text-secondary)", marginTop: "8px" }}>
            Your installed applications.
          </p>
        </div>
        <button 
          className="btn btn-secondary" 
          onClick={handleCheckAllUpdates}
          disabled={isCheckingAllUpdates}
        >
          {isCheckingAllUpdates ? <Loader2 size={16} className="animate-spin" /> : <RefreshCcw size={16} />} 
          Check for Updates
        </button>
      </div>
      
      <div className="content-body">
        {apps.length === 0 ? (
          <div style={{ textAlign: "center", padding: "40px", color: "var(--text-secondary)" }}>
            You haven't installed any apps yet. Head to the Storefront!
          </div>
        ) : (
          <div className="app-grid">
            {apps.map(app => (
              <div 
                key={app.id} 
                className="app-card glass-panel" 
                style={{ 
                  borderTop: app.is_running ? "2px solid var(--accent-success)" : "",
                  cursor: "grab",
                  opacity: draggedAppId === app.id ? 0.5 : 1,
                  transform: draggedAppId === app.id ? "scale(1.02)" : "scale(1)",
                  transition: "opacity 0.2s, transform 0.2s"
                }}
                draggable
                onDragStart={(e) => handleDragStart(e, app.id)}
                onDragOver={handleDragOver}
                onDragEnter={(e) => handleDragEnter(e, app.id)}
                onDragEnd={handleDragEnd}
              >
                <div className="app-card-header">
                  <div className="app-card-title" style={{ display: "flex", alignItems: "center", gap: "8px" }}>
                    <div className={`status-indicator ${app.is_running ? 'status-running' : 'status-stopped'}`}></div>
                    {app.name}
                  </div>
                  <div style={{ display: "flex", gap: "6px" }}>
                    {app.version && (
                      <span className="badge" style={{ backgroundColor: "var(--bg-modifier-hover)", color: "var(--text-primary)" }}>
                        {app.version}
                      </span>
                    )}
                    <span className={`badge ${app.mode === "prod" ? "badge-prod" : "badge-dev"}`}>
                      {app.mode}
                    </span>
                    {updatesAvailable[app.id] && (
                      <span className="badge" style={{ backgroundColor: "var(--accent-warning)", color: "#000" }}>
                        Update: {updatesAvailable[app.id]}
                      </span>
                    )}
                  </div>
                </div>
                
                <div className="app-card-desc">
                  {app.is_running ? "App is running." : "App is currently stopped."}
                </div>
                
                <div className="app-card-actions">
                  {app.is_running ? (
                    <>
                      <button 
                        className="btn btn-primary" 
                        onClick={() => openUrl(`http://localhost:${app.published_port || app.port}`)}
                        disabled={loadingApp[app.id]}
                        title="Open App"
                      >
                        <ExternalLink size={16} /> Open
                      </button>
                      <button 
                        className="btn btn-secondary" 
                        onClick={async () => {
                           await handleStop(app.id);
                           await handleLaunch(app.id);
                        }}
                        disabled={loadingApp[app.id]}
                        title="Restart App"
                      >
                        {loadingApp[app.id] ? <Loader2 size={16} className="animate-spin" /> : <RefreshCw size={16} />} Restart
                      </button>
                      <button 
                        className="btn btn-danger" 
                        onClick={() => handleStop(app.id)}
                        disabled={loadingApp[app.id]}
                        title="Stop App"
                      >
                        {loadingApp[app.id] ? <Loader2 size={16} className="animate-spin" /> : <Square size={16} />} 
                        Stop
                      </button>
                    </>
                  ) : (
                    <button 
                      className="btn btn-success" 
                      onClick={() => handleLaunch(app.id)}
                      disabled={loadingApp[app.id]}
                    >
                      {loadingApp[app.id] ? <Loader2 size={16} className="animate-spin" /> : <Play size={16} />} 
                      Launch
                    </button>
                  )}
                  
                  <button className="btn btn-secondary" onClick={() => handleBackup(app.id)} title="Backup Data">
                    <FolderDown size={16} />
                  </button>
                  
                  {app.mode === "prod" && updatesAvailable[app.id] && (
                    <button 
                      className="btn btn-success" 
                      onClick={() => handleUpdate(app.id)}
                      disabled={checkingUpdate[app.id]}
                      title={`Install update ${updatesAvailable[app.id]}`}
                    >
                      {checkingUpdate[app.id] ? <Loader2 size={16} className="animate-spin" /> : <CloudDownload size={16} />} Update
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </>
  );
}
