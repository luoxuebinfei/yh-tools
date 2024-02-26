use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Window;
use tokio::time;

use std::cmp;

use crate::{db, notify};

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
    pub louzhuregtime: Option<String>,
    pub shijianchuo: i64,
    pub shorttime: String,
    pub title: String,
    pub url: String,
    pub yuanurl: String,
}

#[tauri::command]
pub async fn get_data(window: Window, app: tauri::AppHandle) {
    if R.load(Ordering::Relaxed) {
        let data = read_from_sql().await;
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
            // println!("{}", Local::now());
            window
                .emit("listen_data_time", Local::now().timestamp_millis())
                .unwrap();
            let mut notify_list = vec![];
            if old_list.is_empty() {
                if html.is_empty() {
                    window.emit("listen_data", read_from_sql().await).unwrap();
                } else {
                    window.emit("listen_data", &html).unwrap();
                }
                old_list = html;
                write_to_sql(&old_list).await;
                // println!("old_list: {:?}", &old_list);
            } else {
                // 取差集
                let minusion = html
                    .iter()
                    .filter(|&u| !old_list.contains(u))
                    .collect::<Vec<_>>();
                // println!("minusion: {:?}", minusion);
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
                    // println!("filter: {:?}", c);
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
                        notify_list.push(body);
                        // notify::notify(body, app.clone());
                    }
                });
                write_to_sql(&html).await;
                old_list = html;
            }
            for i in notify_list {
                notify::notify(i, app.clone()).await;
            }
            // 睡眠15秒
            time::sleep(time::Duration::from_secs(15)).await;
        }
    });
    // set.join_next().await;
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

// 写入数据库
async fn write_to_sql(data: &PushList) {
    let db_path = db::get_db_path();
    let conn = rusqlite::Connection::open(db_path).unwrap();
    let mut reverse_data = data.clone();
    reverse_data.reverse();
    for i in reverse_data {
        conn.execute(
            "INSERT OR IGNORE INTO xianbaoku (cateid, comments, content, catename, datetime, id, louzhu, louzhuregtime, shijianchuo, shorttime, title, url, yuanurl) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            &[&i.cateid,&i.comments.to_string(),&i.content,&i.catename,&i.datetime,&i.id.to_string(),&i.louzhu,&i.louzhuregtime.map_or("".to_string(), |x| x.to_string()),&i.shijianchuo.to_string(),&i.shorttime,&i.title,&i.url,&i.yuanurl],
        ).unwrap();
    }
}

// 从数据库读取
async fn read_from_sql() -> PushList {
    let db_path = db::get_db_path();
    let conn = rusqlite::Connection::open(db_path).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM xianbaoku ORDER BY id DESC LIMIT 10")
        .unwrap();
    let push_iter = stmt
        .query_map([], |row| {
            Ok(Push {
                id: row.get("id")?,
                cateid: row.get("cateid")?,
                comments: row.get("comments")?,
                content: row.get("content")?,
                catename: row.get("catename")?,
                datetime: row.get("datetime")?,
                louzhu: row.get("louzhu")?,
                louzhuregtime: row.get("louzhuregtime")?,
                shijianchuo: row.get("shijianchuo")?,
                shorttime: row.get("shorttime")?,
                title: row.get("title")?,
                url: row.get("url")?,
                yuanurl: row.get("yuanurl")?,
            })
        })
        .unwrap();
    let mut pushes = Vec::new();
    for push in push_iter {
        pushes.push(push.unwrap());
    }
    pushes
}

// 读取关键词的json文件
async fn read_keyword() -> Vec<String> {
    let db_path = db::get_db_path();
    let conn = rusqlite::Connection::open(db_path).unwrap();
    let mut stmt = conn
        .prepare("SELECT keyword FROM keywords WHERE belong = 'xianbaoku'")
        .unwrap();
    let keyword_iter = stmt
        .query_map([], |row| Ok(row.get::<usize, String>(0)?))
        .unwrap();
    let mut keywords = Vec::new();
    for keyword in keyword_iter {
        keywords.push(keyword.unwrap());
    }
    keywords
}

// xianbao服务循环不可用时将状态改为false
// #[tauri::command]
// pub async fn check_xianbao_server() {
//     R.store(false, Ordering::Relaxed);
// }
