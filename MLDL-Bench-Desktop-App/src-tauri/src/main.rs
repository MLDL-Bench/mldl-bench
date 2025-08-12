// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mldl_types::{HardwareSummary, RunRequest};

#[tauri::command]
fn get_hardware_summary() -> HardwareSummary {
    mldl_sysinfo::hardware_summary()
}

#[tauri::command]
fn run_suite(req: RunRequest) -> Result<(), String> {
    mldl_runner::run_benchmarks(req).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hardware_summary, run_suite])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

