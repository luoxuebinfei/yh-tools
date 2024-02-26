use std::fs::{self, OpenOptions};

use dirs::config_dir;
use reqwest::header::{self, HeaderMap};
use scraper::{Html, Selector};

use super::{smzdm_struct::Monitor, three_hour_hot::read_smzdm_cookie};

fn get_header() -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.smzdm.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.6".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    // 读取cookie
    headers.insert(header::COOKIE, read_smzdm_cookie().parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert(
        "sec-ch-ua",
        "\"Catsxp\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\""
            .parse()
            .unwrap(),
    );
    // headers.insert("referer", "https://www.smzdm.com/p/95619359/".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67".parse().unwrap());

    headers
}

// 定时任务监控页面变动
pub async fn detect_list(app: tauri::AppHandle) {
    let headers = get_header();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let mut monitor_list = read_file();
    for monitor in monitor_list.iter_mut() {
        let res = client
            .get(monitor.url.clone())
            .headers(headers.clone())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let document = Html::parse_document(&res);
        let selector = Selector::parse("div#feed-main").unwrap();
        for element in document.select(&selector) {
            let selector = Selector::parse("h1.title.J_title").unwrap();
            for element in element.select(&selector) {
                monitor.title = element.text().collect::<String>().trim().to_string();
            }
            let selector = Selector::parse("article.txt-detail").unwrap();
            for element in element.select(&selector) {
                if element.text().collect::<String>().trim().to_string() != monitor.content {
                    monitor.content = element.text().collect::<String>().trim().to_string();
                    monitor.is_update = true;
                } else {
                    monitor.is_update = false;
                    monitor.content = element.text().collect::<String>().trim().to_string();
                }
            }
            let selector = Selector::parse("span.old").unwrap();
            if element.select(&selector).count() > 0 {
                monitor.is_expired = true;
            } else {
                monitor.is_expired = false;
            }
        }
    }
    for monitor in monitor_list.iter() {
        if monitor.is_update && !monitor.is_expired {
            let _ =
                tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
                    .title("什么值得买页面监控")
                    .body("您订阅的页面有更新！")
                    .sound("Default") //系统默认提示音
                    .show();
        }
    }
    write_file(monitor_list);
}

// 获取一条新记录的信息
pub async fn get_monitor_info(url: String) {
    let headers = get_header();
    let mut monitor = Monitor {
        url,
        title: "".to_string(),
        content: "".to_string(),
        is_update: false,
        is_expired: false,
    };
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get(monitor.url.clone())
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = Html::parse_document(&res);
    let selector = Selector::parse("div.main-left").unwrap();
    for element in document.select(&selector) {
        let selector = Selector::parse("h1.title.J_title").unwrap();
        for element in element.select(&selector) {
            monitor.title = element.text().collect::<String>().trim().to_string();
        }
        let selector = Selector::parse("article.txt-detail").unwrap();
        for element in element.select(&selector) {
            monitor.content = element.text().collect::<String>().trim().to_string();
            // if element.inner_html() != monitor.content {
            //     monitor.content = element.inner_html();
            //     monitor.is_update = true;
            // } else {
            //     monitor.is_update = false;
            //     monitor.content = element.inner_html();
            // }
        }
        let selector = Selector::parse("span.old").unwrap();
        if element.select(&selector).count() > 0 {
            monitor.is_expired = true;
        } else {
            monitor.is_expired = false;
        }
    }
    appand_file(monitor);
}

fn read_file() -> Vec<Monitor> {
    let file_path = config_dir().unwrap().join("com.yhtools").join("smzdm_monitor.json").to_str().unwrap().to_string();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    match serde_json::from_reader(file) {
        Ok(data) => data,
        Err(e) => {
            if e.is_eof() {
                Vec::new()
            } else if e.is_data() {
                Vec::new()
            } else {
                panic!("{}", e);
            }
        }
    }
}

fn write_file(data: Vec<Monitor>) {
    let file_path = config_dir().unwrap().join("com.yhtools").join("smzdm_monitor.json").to_str().unwrap().to_string();
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .unwrap();
    serde_json::to_writer(file, &data).unwrap();
}

fn appand_file(data: Monitor) {
    let mut monitor_list = read_file();
    monitor_list.push(data);
    write_file(monitor_list);
}

#[tauri::command]
pub async fn get_monitor_list() -> Result<Vec<Monitor>, String> {
    Ok(read_file())
}

// #[tauri::command]
// pub async fn add_monitor(url: String) -> Result<(), String> {
//     get_monitor_info(url).await;
//     Ok(())
// }

#[tauri::command]
pub async fn edit_monitor(command: String, url: String) -> Result<(), String> {
    let mut monitor_list = read_file();
    if command == "add" {
        get_monitor_info(url).await;
        return Ok(())
    }else if command == "del"{
        monitor_list.retain(|x| x.url != url);
    } else if command == "click"{
        for monitor in monitor_list.iter_mut() {
            if monitor.url == url {
                monitor.is_update = false;
            }
        }
    }
    write_file(monitor_list);
    Ok(())
}
