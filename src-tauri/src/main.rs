// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_bash_script])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn run_bash_script(script_path: String) -> Result<String, String> {
  let output = match Command::new("bash")
    .arg(script_path)  // Use the provided script path
    .output() {
      Ok(output) => output,
      Err(err) => return Err(format!("Failed to run script: {}", err)),
  };

  let output_string = String::from_utf8(output.stdout)
    .map_err(|err| format!("Failed to convert output to string: {}", err))?;

  println!("Script output: {}", output_string);
  Ok(output_string)
}

