# Lucas' PhD Toolbox

A cross-platform desktop launcher for managing and running your personal web applications — built with **Tauri**, **React**, and **Rust**.

Lucas' PhD Toolbox lets you install locally hosted web apps from GitHub Releases from a single clean interface.

---

## Features

- **📦 Storefront** — Browse and install production apps directly from GitHub Releases. The launcher automatically detects your OS and architecture and downloads the correct binary.
- **📚 Library** — Launch, stop, restart, and open your installed apps. Drag and drop to reorder widgets. Displays the current installed version of each app.
- **🔄 Auto-updates** — Each app displays its installed version. Click "Check for Update" to compare against the latest GitHub Release and update with one click.
- **💾 Backup System** — Automatically syncs app backups to a configurable master backup directory (e.g. Dropbox), keeping it in sync as older backups are rotated out.
- **⚙️ Settings** — Configure your master backup directory and check for new launcher releases from within the app.
- **🔒 Privacy-first** — The app is running entirely locally, nothing is sent to the cloud.

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

- `app-windows.exe` — Windows

---
