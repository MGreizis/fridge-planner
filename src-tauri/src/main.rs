// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod window;

use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(app: AppHandle, name: &str) -> String {
  app.emit_all("logging", "loggingPayload").unwrap();
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn placeholder(text: String) -> String {
    format!("This is placeholder text: `{}`, funny, right?", text)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        greet,
        placeholder,
        window::create_window,
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
