# Lucas' PhD Toolbox

A cross-platform desktop launcher for managing and running your personal web applications — built with **Tauri**, **React**, and **Rust**.

Lucas' PhD Toolbox lets you install production web apps from GitHub Releases and run local dev apps side by side, all from a single clean interface.

---

## Features

- **📦 Storefront** — Browse and install production apps directly from GitHub Releases. The launcher automatically detects your OS and architecture and downloads the correct binary.
- **📚 Library** — Launch, stop, restart, and open your installed apps. Drag and drop to reorder widgets. Displays the current installed version of each app.
- **🔄 Auto-updates** — Each app displays its installed version. Click "Check for Update" to compare against the latest GitHub Release and update with one click.
- **💾 Backup System** — Automatically syncs app backups to a configurable master backup directory (e.g. Dropbox), keeping it in sync as older backups are rotated out.
- **⚙️ Settings** — Configure your master backup directory and check for new launcher releases from within the app.
- **🔒 Privacy-first** — No sensitive data is stored in the codebase. Dev environment paths are loaded from a local config file that is never committed to git.

---

## Available Tools

The launcher comes pre-configured with a catalog of the following productivity and research applications:

- **[Job Application Tracker](https://github.com/LucasFenaux/job-application-tracker)** — A localized database to track job applications, companies, and interview statuses.
- **[Literature Map](https://github.com/LucasFenaux/literature-map)** — A visual node-graph tool for academic literature review and exploring connections between research papers.
- **[Portfolio Analytics](https://github.com/LucasFenaux/portfolio-analytics)** — Financial tracking and analytics dashboard for personal investment portfolios.
- **[Core Planner](https://github.com/LucasFenaux/core-planner)** — An all-in-one productivity suite, daily planner, and task manager.

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop shell | [Tauri 2](https://tauri.app/) |
| Frontend | React 19 + TypeScript + Vite |
| Backend | Rust |
| Styling | Vanilla CSS |

---

## Installing a Production Release

Go to the [Releases](../../releases) page and download the installer for your platform:

| Platform | File |
|----------|------|
| macOS (Apple Silicon + Intel) | `.dmg` (universal binary) |
| Windows | `.exe` (installer) |
| Linux | `.AppImage` |

---

## Running Locally (Development)

**Prerequisites:** [Rust](https://rustup.rs/), [Node.js 20+](https://nodejs.org/), and the [Tauri CLI prerequisites](https://tauri.app/start/prerequisites/) for your OS.

```bash
# Install frontend dependencies
npm install

# Start the Tauri dev server (hot-reloads both frontend and Rust backend)
npm run tauri dev
```

### Configuring Dev Apps

When running in debug mode, the launcher loads dev app definitions from a local file that is **not** committed to the repository:

```
~/Library/Application Support/WebAppLauncher/dev_apps.json   # macOS
~/.local/share/WebAppLauncher/dev_apps.json                   # Linux
%APPDATA%\WebAppLauncher\dev_apps.json                        # Windows
```

Create this file with an array of app definitions. Example:

```json
[
  {
    "id": "my-dev-app",
    "name": "My App",
    "description": "Local dev build",
    "repo_url": "/absolute/path/to/your/app",
    "port": 3000,
    "published_port": null,
    "mode": "dev",
    "is_installed": false,
    "is_running": false,
    "version": "dev"
  }
]
```

The launcher will call `bash start.sh` inside the `repo_url` directory, passing `PORT` and `BACKEND_PORT` as environment variables.

---

## Building a Release

Releases are built automatically via GitHub Actions when you push a `v*` tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

This will compile native installers for macOS (universal), Windows, and Linux and attach them to a GitHub Release.

---

## Adding Apps to the Storefront

The app catalog is defined in [`src-tauri/src/lib.rs`](src-tauri/src/lib.rs) in the `get_catalog_base()` function. Each app needs a GitHub repository with releases that include named binaries in the format:

- `app-macos` / `app-macos-arm64` — macOS
- `app-linux` — Linux  
- `app-windows.exe` — Windows

---

## License

MIT
