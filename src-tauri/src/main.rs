// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xianbaofun;

mod tray;
mod smzdm;

use chrono::Local;
use tauri::{api::notification::Notification, async_runtime::block_on};
use tokio::time;
use tokio_cron_scheduler::{JobScheduler, Job};
use xianbaofun::*;

use crate::utils::set_window_shadow;

mod notify;
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
        window
            .emit(
                "test",
                r#"<span class="text-red-600 text-9xl bg-red-600">test</span>"#,
            )
            .unwrap();
    }
}

#[tauri::command]
async fn test2(_app: tauri::AppHandle) {
    time::sleep(time::Duration::from_secs(5)).await;
    notify::is_full_screen();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, cwd| {
            // 防止重复运行程序
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("该程序正在运行中，请不要再次启动！")
                .body(cwd)
                .sound("Default")
                .show()
                .unwrap();
        }))
        .setup(|app| {
            set_window_shadow(app);
            let _sched = block_on(initialize_cron_scheduler(app.handle()));
            Ok(())
        })
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            greet,
            create_window,
            get_data,
            xianbaofun::return_keyword,
            xianbaofun::change_keyword,
            notify::change_hover_status,
            test2,
            smzdm::three_hour_hot::smzdm_3hhot,
            smzdm::three_hour_hot::smzdm_3hhot_more,
            smzdm::three_hour_hot::return_smzdm_keyword,
            smzdm::three_hour_hot::change_smzdm_keyword
        ])
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

async fn initialize_cron_scheduler(app: tauri::AppHandle){
    let cron_handler = JobScheduler::new().await.expect("Failed to initialize tokio cron handler");
    cron_handler.start().await.expect("Failed to start tokio cron handler");
    let heart_job = Job::new_cron_job("0 0/30 * * * ?",move |_uuid,_i|{
        let app = app.clone();
        let task = async move {
            println!("定时任务：{}", Local::now());
            smzdm::three_hour_hot::timing_task_smzdm_3hhot(app).await;
        };
        tokio::spawn(task);
    }).unwrap();
    let _job_id = cron_handler.add(heart_job).await.unwrap();
}
