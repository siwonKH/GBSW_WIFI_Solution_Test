// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, CustomMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();

                    println!("{}", window.is_visible().unwrap());
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        let _ = window.move_window(Position::TrayCenter);
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app

                println!("Out side click");
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
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
