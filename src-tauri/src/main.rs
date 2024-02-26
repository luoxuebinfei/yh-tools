// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod xianbaofun;

mod tray;
mod smzdm;
mod db;

use std::io;

use chrono::Local;
use tauri::{api::notification::Notification, async_runtime::block_on};
use tokio::time;
use tokio_cron_scheduler::{JobScheduler, Job};
use tracing::Level;
use tracing_subscriber::fmt::{time::FormatTime, format::Writer};
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
    // time::sleep(time::Duration::from_secs(5)).await;
    // notify::is_full_screen();
    let a = smzdm::search::search_keyword("可乐".to_string(),true).await;
    match a {
        Ok(sl) => {
            println!("{:?}", sl);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

// 用来格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

fn main() {
    let file_appender = tracing_appender::rolling::daily("./logs", "tracing.log");
    // 定义日志文件滚动策略，将文件名中的日期放在后面
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 设置日志输出时的格式，例如，是否包含日志级别、是否包含日志来源位置、设置日志的时间格式
    // 参考: https://docs.rs/tracing-subscriber/0.3.3/tracing_subscriber/fmt/struct.SubscriberBuilder.html#method.with_timer
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);

    // 初始化并设置日志格式(定制和筛选日志)
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(io::stdout) // 写入标准输出
        .with_writer(non_blocking) // 写入文件，将覆盖上面的标准输出
        .with_ansi(false) // 如果日志是写入文件，应将ansi的颜色输出功能关掉
        .event_format(format)
        .init(); // 初始化并将SubScriber设置为全局SubScriber

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
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            set_window_shadow(app);
            let _sched = block_on(initialize_cron_scheduler(app.handle()));
            db::init();
            Ok(())
        })
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            greet,
            create_window,
            get_data,
            notify::change_hover_status,
            test2,
            smzdm::three_hour_hot::smzdm_3hhot,
            smzdm::three_hour_hot::smzdm_3hhot_more,
            smzdm::three_hour_hot::return_smzdm_keyword,
            smzdm::three_hour_hot::change_smzdm_keyword,
            smzdm::search::smzdm_search,
            smzdm::search::smzdm_write_cookies,
            smzdm::monitoring::get_monitor_list,
            smzdm::monitoring::edit_monitor,
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
    let app1 = app.clone();
    let heart_job = Job::new_cron_job("0 0/30 * * * ?",move |_uuid,_i|{
        let app = app1.clone();
        let task = async move {
            println!("定时任务：{}", Local::now());
            smzdm::three_hour_hot::timing_task_smzdm_3hhot(app).await;
        };
        tokio::spawn(task);
    }).unwrap();
    let _job_id = cron_handler.add(heart_job).await.unwrap();
    // 什么值得买页面监控任务
    let job2 = Job::new_cron_job("0 0/10 * * * ?",move |_uuid,_i|{
        let app = app.clone();
        let task = async move {
            println!("定时任务2：{}", Local::now());
            smzdm::monitoring::detect_list(app).await;
        };
        tokio::spawn(task);
    });
    match job2 {
        Ok(j) => {
            let _job_id2 = cron_handler.add(j).await.unwrap();
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
    
}
