import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { LayoutGrid, Library, Settings as SettingsIcon } from "lucide-react";
import Storefront from "./pages/Storefront";
import LibraryPage from "./pages/Library";
import SettingsPage from "./pages/Settings";
import "./App.css";

type Tab = "storefront" | "library" | "settings";

function App() {
  const [activeTab, setActiveTab] = useState<Tab>("library");
  const [depsOk, setDepsOk] = useState<boolean>(true);
  const [depsMessage, setDepsMessage] = useState<string>("");

  useEffect(() => {
    async function checkDeps() {
      try {
        const res = await invoke("check_dependencies") as { ok: boolean, message: string };
        setDepsOk(res.ok);
        setDepsMessage(res.message);
      } catch (err) {
        console.error("Failed to check dependencies", err);
      }
    }
    checkDeps();
  }, []);

  return (
    <div className="app-container">
      {/* Sidebar */}
      <div className="sidebar">
        <div className="sidebar-header">
          <div className="status-indicator status-running"></div>
          Lucas' PhD Toolbox
        </div>

        <div
          className={`nav-item ${activeTab === "library" ? "active" : ""}`}
          onClick={() => setActiveTab("library")}
        >
          <Library size={20} />
          Library
        </div>

        <div
          className={`nav-item ${activeTab === "storefront" ? "active" : ""}`}
          onClick={() => setActiveTab("storefront")}
        >
          <LayoutGrid size={20} />
          Storefront
        </div>

        <div style={{ flex: 1 }}></div>

        <div
          className={`nav-item ${activeTab === "settings" ? "active" : ""}`}
          onClick={() => setActiveTab("settings")}
        >
          <SettingsIcon size={20} />
          Settings
        </div>
      </div>

      {/* Main Content */}
      <div className="main-content">
        {!depsOk && (
          <div style={{ background: "var(--accent-danger)", color: "white", padding: "12px", textAlign: "center", fontWeight: "600" }}>
            Warning: {depsMessage}
          </div>
        )}

        <div className="animate-fade-in" style={{ flex: 1, display: "flex", flexDirection: "column" }}>
          {activeTab === "storefront" && <Storefront />}
          {activeTab === "library" && <LibraryPage />}
          {activeTab === "settings" && <SettingsPage />}
        </div>
      </div>
    </div>
  );
}

export default App;
