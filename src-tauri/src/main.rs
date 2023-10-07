// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xianbaofun;

mod tray;
use tokio::time;
use xianbaofun::*;

use crate::utils::set_window_shadow;

mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn create_window(app: tauri::AppHandle) {
    let window = tauri::WindowBuilder::new(&app, "test", tauri::WindowUrl::App("/notify".into()))
        .title("test")
        .transparent(true)
        .decorations(false)
        .position(1590.0, 942.0)
        .inner_size(320.0, 80.0)
        .resizable(false)
        .build()
        .unwrap();
    // time::sleep(time::Duration::from_secs(2)).await;
    // let _ = window.close();
    // let a = match window.current_monitor() {
    //     Ok(m) => {
    //         m
    //     },
    //     Err(_) => {
    //         println!("error");
    //         return;
    //     }
    // };
    // println!("{:?}", a);
    // let main_window = app.get_window("main").unwrap();
    // let main_screen = main_window.current_monitor().unwrap();
    // print!("main_screen: {:?}", main_screen);
    // time::sleep(time::Duration::from_millis(500)).await;
    for _i in 0..10 {
        time::sleep(time::Duration::from_millis(100)).await;
        window.emit("test", "test").unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_window_shadow(app);
            Ok(())
        })
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![greet, create_window, get_data])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
