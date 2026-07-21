import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Download, CheckCircle, Loader2 } from "lucide-react";
import { AppInfo } from "../types";

export default function Storefront() {
  const [catalog, setCatalog] = useState<AppInfo[]>([]);
  const [installing, setInstalling] = useState<Record<string, boolean>>({});

  useEffect(() => {
    async function fetchCatalog() {
      try {
        const apps: AppInfo[] = await invoke("get_catalog");
        setCatalog(apps);
      } catch (err) {
        console.error("Failed to fetch catalog:", err);
      }
    }
    fetchCatalog();
  }, []);

  const handleInstall = async (appId: string) => {
    setInstalling({ ...installing, [appId]: true });
    try {
      await invoke("install_app", { appId });
      // Update local state to show it's installed
      setCatalog(catalog.map(app => 
        app.id === appId ? { ...app, is_installed: true } : app
      ));
    } catch (err) {
      console.error("Failed to install app:", err);
      alert(`Installation failed: ${err}`);
    } finally {
      setInstalling({ ...installing, [appId]: false });
    }
  };

  return (
    <>
      <div className="content-header">
        <h1>Storefront</h1>
        <p style={{ color: "var(--text-secondary)", marginTop: "8px" }}>
          Discover and install local web applications.
        </p>
      </div>
      
      <div className="content-body">
        <div className="app-grid">
          {catalog.map(app => (
            <div key={app.id} className="app-card glass-panel">
              <div className="app-card-header">
                <div className="app-card-title">{app.name}</div>
                <span className={`badge ${app.mode === "prod" ? "badge-prod" : "badge-dev"}`}>
                  {app.mode}
                </span>
              </div>
              
              <div className="app-card-desc">
                {app.description}
              </div>
              
              <div className="app-card-actions">
                {app.is_installed ? (
                  <button className="btn btn-secondary" disabled>
                    <CheckCircle size={16} /> Installed
                  </button>
                ) : (
                  <button 
                    className="btn btn-primary" 
                    onClick={() => handleInstall(app.id)}
                    disabled={installing[app.id]}
                  >
                    {installing[app.id] ? (
                      <><Loader2 size={16} className="animate-spin" /> Installing...</>
                    ) : (
                      <><Download size={16} /> Install</>
                    )}
                  </button>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
