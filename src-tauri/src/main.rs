#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod dto;
mod client;

use client::tenor_client::tenor_client;

fn main() {
  tenor_client();
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}!", name)
}
