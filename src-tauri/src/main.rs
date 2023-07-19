// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn pass_add_msg() -> String {
    ("Password Entry Added").into()
}

#[tauri::command]
fn unlock_msg() -> String {
    ("Unlocked").into()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![pass_add_msg,unlock_msg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
