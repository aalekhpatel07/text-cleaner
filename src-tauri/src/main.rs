#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
  match name.contains(' ') {
    false => Ok(format!("Hello, {}", name)),
    true => Err("Name should not contain spaces".to_string())
  }
}