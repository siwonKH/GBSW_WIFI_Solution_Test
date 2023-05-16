// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            connect_to_wifi,
            disconnect_from_wifi,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn connect_to_wifi(ssid: String) -> Result<String, String> {
    let output = std::process::Command::new("netsh")
        .arg("wlan")
        .arg("connect")
        .arg(ssid)
        .output()
        .map_err(|e| format!("failed to execute process: {}", e))?;

    if output.status.success() {
        Ok("Sent connection request to WIFI.".to_string())
    } else {
        Err("failed to connect to the Wi-Fi network".to_string())
    }
}

#[tauri::command]
fn disconnect_from_wifi() -> Result<String, String> {
    let output = std::process::Command::new("netsh")
        .arg("wlan")
        .arg("disconnect")
        .output()
        .map_err(|e| format!("failed to execute process: {}", e))?;

    if output.status.success() {
        Ok("Disconnected".to_string())
    } else {
        Err("failed to disconnect from the Wi-Fi network".to_string())
    }
}

//// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//#[tauri::command]
//fn greet(name: &str) -> String {
//    format!("Hello, {}! You've been greeted from Rust!", name)
//}
//
//fn main() {
//    tauri::Builder::default()
//        .invoke_handler(tauri::generate_handler![greet])
//        .run(tauri::generate_context!())
//        .expect("error while running tauri application");
//}
