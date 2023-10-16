// 通知模块

use core::time;
use chrono::Local;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};
use tauri::UserAttentionType;

use crate::xianbaofun::Push;

// 鼠标是否移入通知框的状态变量
static ISHOVER: AtomicBool = AtomicBool::new(false);

pub async fn notify(body: Push, app: tauri::AppHandle) {
    // 生成随机的lable_name
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let content_num = body.content.chars().count();
    let add_height = content_num / 25 * 16;
    let full_screen = is_full_screen(); //获取是否有全屏应用，有就不置顶通知
    let window = tauri::WindowBuilder::new(
        &app,
        rand_string.clone(),
        tauri::WindowUrl::App("/notify".into()),
    )
    .title("test")
    .transparent(true)
    .decorations(false)
    .always_on_top(!full_screen)
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
    let start_time = Local::now().timestamp_millis();
    while ISHOVER.load(Ordering::Relaxed) && Local::now().timestamp_millis() - start_time < 20000 {}
    ISHOVER.store(false, Ordering::Relaxed);
    thread::sleep(time::Duration::from_millis(500));
    let _ = window.close().unwrap();
}

// 前端发送鼠标是否移入/移出通知框的事件
#[tauri::command]
pub async fn change_hover_status(param: bool) {
    ISHOVER.store(param, Ordering::Relaxed);
}

use windows::Win32::UI::WindowsAndMessaging::*;

// 全屏判断
pub fn is_full_screen() -> bool {
    unsafe {
        //获取焦点窗口
        let h = GetForegroundWindow();
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(h, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize]); // 转换窗口名称
        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        // 获取窗口信息
        GetWindowInfo(h, &mut info).unwrap();
        let w = GetSystemMetrics(SM_CXSCREEN); // 屏幕的宽
        let h = GetSystemMetrics(SM_CYSCREEN); // 屏幕的高
        
        if info.rcWindow.left == 0 && info.rcWindow.top == 0 && info.rcWindow.right == w && info.rcWindow.bottom == h {
            println!("{}: 全屏占用", text);
            return true;
        }
        false
    }
}
