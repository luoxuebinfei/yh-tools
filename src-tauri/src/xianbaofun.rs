use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{Ordering, AtomicBool};
use tauri::Window;
use tokio::{
    fs::{self, OpenOptions},
    time,
};

use std::cmp;

use crate::notify;

// 作为是否已有线程访问线报库的全局变量
static R: AtomicBool = AtomicBool::new(false);

pub type PushList = Vec<Push>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Push {
    pub cateid: String,
    pub comments: i64,
    pub content: String,
    pub catename: String,
    pub datetime: String,
    pub id: i64,
    pub louzhu: String,
    pub louzhuregtime: Option<serde_json::Value>,
    pub shijianchuo: i64,
    pub shorttime: String,
    pub title: String,
    pub url: String,
    pub yuanurl: String,
}

#[tauri::command]
pub async fn get_data(window: Window, app: tauri::AppHandle) {
    if R.load(Ordering::Relaxed) {
        let data = read_to_file().await;
        window.emit("listen_data", &data).unwrap();
        return;
    }
    // 克隆window以供后续使用
    let window_clone = window.clone();
    let mut set = tokio::task::JoinSet::new();
    let _a = set.spawn(async move {
        R.store(true, Ordering::Relaxed);
        let mut old_list = Vec::new();
        loop {
            let html = reqwest::get("http://new.xianbao.fun/plus/json/push.json")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let html: PushList = match serde_json::from_str(&html) {
                Ok(v) => v,
                Err(e) => {
                    println!("error: {:?}", e);
                    continue;
                }
            };
            println!("{}", Local::now());
            window.emit("listen_data_time", Local::now().timestamp_millis()).unwrap();
            if old_list.is_empty() {
                window.emit("listen_data", &html).unwrap();
                old_list = html;
                write_to_file(&old_list).await;
                println!("old_list: {:?}", &old_list);
            } else {
                // 取差集
                let minusion = html
                    .iter()
                    .filter(|&u| !old_list.contains(u))
                    .collect::<Vec<_>>();
                println!("minusion: {:?}", minusion);
                window.emit("listen_data", &minusion).unwrap();
                let b = read_keyword().await;
                // 对比消息相似度，如果大于0.8则发送通知
                minusion.into_iter().for_each(|u| {
                    let mut a = 0;
                    for i in old_list.iter() {
                        let sim = sim_jaro(&u.title, &i.title);
                        if sim > 0.8 {
                            a += 1;
                        }
                    }
                    // let b = vec![String::from("水"), String::from("猫超"), String::from("JD"),String::from("美团"),String::from("拼多多")];
                    let c = b
                        .iter()
                        .filter(|x| u.title.contains(&**x))
                        .collect::<Vec<_>>();
                    let d = b
                        .iter()
                        .filter(|x| u.content.contains(&**x))
                        .collect::<Vec<_>>();
                    println!("filter: {:?}", c);
                    if a > 0 || !c.is_empty() || !d.is_empty() {
                        let mut body = u.clone();
                        for i in c {
                            body.title = body
                                .title
                                .replace(&*i, &format!("<span class=\"text-red-600\">{i}</span>"));
                        }
                        for i in d {
                            body.content = body
                                .content
                                .replace(&*i, &format!("<span class=\"text-red-600\">{i}</span>"));
                        }

                        notify::notify(body, app.clone());
                    }
                });
                write_to_file(&html).await;
                old_list = html;
            }
            // 睡眠15秒
            time::sleep(time::Duration::from_secs(15)).await;
        }
    });
    while let Some(_) = set.join_next().await {
        // 当线程意外退出时，将R变量设置为false
        R.store(false, Ordering::Relaxed);
        // 发送一个通知，让前端刷新页面重启线程
        window_clone.emit("xianbao_server_close", true).unwrap();
    }
}

/// jaro similarity 字符串相似度算法
pub fn sim_jaro(s1: &str, s2: &str) -> f64 {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();
    if s1_len == 0 && s2_len == 0 {
        return 1.0;
    }

    let match_distance = cmp::max(s1_len, s2_len) / 2 - 1;
    let mut s1_matches = vec![false; s1_len];
    let mut s2_matches = vec![false; s2_len];
    let mut m: isize = 0;
    for i in 0..s1_len {
        let start = cmp::max(0, i as isize - match_distance as isize) as usize;
        let end = cmp::min(i + match_distance + 1, s2_len);
        for j in start..end {
            if !s2_matches[j] && s1.chars().nth(i) == s2.chars().nth(j) {
                s1_matches[i] = true;
                s2_matches[j] = true;
                m += 1;
                break;
            }
        }
    }
    if m == 0 {
        return 0.0;
    }
    let mut t = 0.0;
    let mut k = 0;
    for i in 0..s1_len {
        if s1_matches[i] {
            while !s2_matches[k] {
                k += 1;
            }
            if s1.chars().nth(i) != s2.chars().nth(k) {
                t += 0.5;
            }
            k += 1;
        }
    }

    let m = m as f64;
    (m / s1_len as f64 + m / s2_len as f64 + (m - t) / m) / 3.0
}

// 写入json文件
async fn write_to_file(data: &PushList) {
    let file_path = r".\data\xianbaofun.json";
    let _ = fs::create_dir_all(r".\data").await;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .await
        .unwrap()
        .into_std()
        .await;
    serde_json::to_writer(file, &data).unwrap();
}

// 读取json文件
async fn read_to_file() -> PushList {
    let file_path = r".\data\xianbaofun.json";
    let file = OpenOptions::new()
        .read(true)
        .open(file_path)
        .await
        .unwrap()
        .into_std()
        .await;
    serde_json::from_reader(file).unwrap()
}

// 读取关键词的json文件
async fn read_keyword() -> Vec<String> {
    let file_path = r".\data\xianbacfun_keyword.json";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .await
        .unwrap()
        .into_std()
        .await;
    match serde_json::from_reader(file) {
        Ok(data) => data,
        Err(e) => {
            if e.is_eof() {
                vec![]
            } else {
                panic!("{}", e);
            }
        }
    }
}

// 写入关键词的json文件
async fn write_keyword(data: Vec<String>) {
    let file_path = r".\data\xianbacfun_keyword.json";
    let _ = fs::create_dir_all(r".\data").await;
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .await
        .unwrap()
        .into_std()
        .await;
    serde_json::to_writer(file, &data).unwrap();
}

// 从文件中读取关键词
#[tauri::command]
pub async fn return_keyword() -> Vec<String> {
    read_keyword().await
}

// 改变关键词时写入文件
#[tauri::command]
pub async fn change_keyword(params: Vec<String>) {
    write_keyword(params).await;
}

// xianbao服务循环不可用时将状态改为false
// #[tauri::command]
// pub async fn check_xianbao_server() {
//     R.store(false, Ordering::Relaxed);
// }