# Desktop App (Tauri) Skeleton

This folder will host the Tauri-based desktop orchestrator (Rust + webview). To keep early builds lean, it is not included in the root Rust workspace yet.

Planned layout:

- `apps/desktop/tauri/` — Tauri project (Rust + frontend)
- Orchestrator will consume crates from `crates/` (e.g., `mldl-sysinfo`, `mldl-exec`)

For now, this is a placeholder per PRD-0001.


