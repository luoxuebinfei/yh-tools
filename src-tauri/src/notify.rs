// 通知模块

use rand::{distributions::Alphanumeric, thread_rng, Rng};
use core::time;
use std::{
    sync::atomic::{Ordering, AtomicBool},
    thread,
};
use tauri::UserAttentionType;

use crate::xianbaofun::Push;

// 鼠标是否移入通知框的状态变量
static ISHOVER: AtomicBool = AtomicBool::new(false);

pub fn notify(body:Push, app: tauri::AppHandle) {
    // 生成随机的lable_name
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let content_num = body.content.chars().count();
    let add_height = content_num / 25 * 16;
    let window = tauri::WindowBuilder::new(
        &app,
        rand_string.clone(),
        tauri::WindowUrl::App("/notify".into()),
    )
    .title("test")
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .position(1590.0, 942.0 - add_height as f64)
    .inner_size(320.0, 80.0 + add_height as f64)
    .resizable(false)
    .build()
    .unwrap();
    for _i in 0..10 {
        // let _ = time::sleep(time::Duration::from_millis(100));
        thread::sleep(time::Duration::from_millis(100));
        window.emit("body", &body).unwrap();
    }
    for _i in 0..10 {
        thread::sleep(time::Duration::from_millis(100));
        window.emit("label_name", &rand_string).unwrap();
    }

    let _ = window
        .request_user_attention(Some(UserAttentionType::Critical))
        .unwrap();
    thread::sleep(time::Duration::from_secs(5));
    while ISHOVER.load(Ordering::Relaxed) {}
    thread::sleep(time::Duration::from_millis(500));
    let _ = window.close();
}

// 前端发送鼠标是否移入/移出通知框的事件
#[tauri::command]
pub async fn change_hover_status(param: bool) {
    ISHOVER.store(param, Ordering::Relaxed);
}