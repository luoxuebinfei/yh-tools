use chrono::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};
use tauri::{UserAttentionType, Window};
use tokio::{
    fs::{self, OpenOptions},
    time,
};

use std::cmp;

// 作为是否已有线程访问线报库的全局变量
static R: AtomicU64 = AtomicU64::new(0);

pub type PushList = Vec<Push>;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Push {
    cateid: String,
    catename: String,
    comments: i64,
    content: String,
    datetime: String,
    id: i64,
    louzhu: String,
    louzhuregtime: Option<serde_json::Value>,
    shijianchuo: i64,
    shorttime: String,
    title: String,
    url: String,
    yuanurl: String,
}

#[tauri::command]
pub async fn get_data(window: Window, app: tauri::AppHandle) {
    if R.load(Ordering::Relaxed) != 0 {
        let data = read_to_file().await;
        window.emit("listen_data", &data).unwrap();
        return;
    }

    tokio::spawn(async move {
        R.fetch_add(1, Ordering::Relaxed);
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
                    let c = b.iter().filter(|x| u.title.contains(&**x)).collect::<Vec<_>>();
                    let d = b.iter().filter(|x| u.content.contains(&**x)).collect::<Vec<_>>();
                    println!("filter: {:?}", c);
                    if a > 0 || !c.is_empty() || !d.is_empty() {
                        let mut body = u.clone();
                        for i in c {
                            body.title = body.title.replace(&*i, &format!("<span class=\"text-red-600\">{i}</span>"));
                        }
                        for i in d {
                            body.content = body.content.replace(&*i, &format!("<span class=\"text-red-600\">{i}</span>"));
                        }
                        
                        notify(body, app.clone());
                    }
                });
                write_to_file(&html).await;
                old_list = html;
            }
            // 睡眠15秒
            time::sleep(time::Duration::from_secs(15)).await;
        }
    });
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

pub fn notify(body:Push, app: tauri::AppHandle) {
    // 生成随机的lable_name
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let content_num = body.content.chars().count();
    let add_height = content_num / 25 * 16;
    // if add_height > 20 {
    //     add_height = add_height - 20;
    // }
    // let body = body.clone();
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
    let _ = window.close();
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
        Ok(data) => {
            data
        },
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

#[tauri::command]
pub async fn return_keyword() -> Vec<String> {
    read_keyword().await
}

#[tauri::command]
pub async fn change_keyword(params: Vec<String>) {
    write_keyword(params).await;
}