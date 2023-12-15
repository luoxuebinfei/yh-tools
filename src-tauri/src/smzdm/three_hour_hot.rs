use std::{
    fs::{self, OpenOptions},
    io::Read,
    vec,
};

use crate::smzdm::smzdm_struct::*;
use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};

// use tokio::fs::{self, OpenOptions};

// 初始化
fn init_smzdm() -> Smzdm {
    return Smzdm {
        article_id: 0,
        article_url: String::new(),
        article_pic_url: String::new(),
        article_pic_style: String::new(),
        article_title: String::new(),
        article_price: String::new(),
        article_referrals: String::new(),
        article_avatar: String::new(),
        article_avatar_url: String::new(),
        article_display_date: String::new(),
        article_date: String::new(),
        article_content: String::new(),
        article_yh_type: String::new(),
        article_mall: String::new(),
        article_mall_url: String::new(),
        article_link: String::new(),
        article_rating: 0,
        article_rating_down: 0,
        article_collect: 0,
        article_comment: 0,
        zhifa_tag: ZhifaTag {
            name: String::new(),
            url: String::new(),
        },
        article_border_style: String::new(),
        cates_str: String::new(),
        cates: Vec::new(),
        brand: String::new(),
    };
}

// 获取三小时热门榜列表
async fn get_three_hour_hot_list() -> Result<Vec<Smzdm>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.smzdm.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.7".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    // 读取cookie
    headers.insert(header::COOKIE, read_smzdm_cookie().parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("referer", "https://search.smzdm.com/".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67".parse().unwrap());

    let mut sl = SmzdmList::new();

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://faxian.smzdm.com/h2s0t0f0c1p1/#filter-block")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    // info!("获取三小时热门榜列表：{}", res.status());
    // let res = res.text().await?;
    if res == "" {
        return Err("获取三小时热门榜列表失败,遇到机器人检测...".into());
    }
    let document = Html::parse_document(&res);
    let selector = Selector::parse("ul#feed-main-list > li").unwrap();
    for element in document.select(&selector).into_iter() {
        let mut s = init_smzdm();
        // 提取文章图片和商城
        let selector = Selector::parse("div.feed-ver-pic > a").unwrap();
        for element in element.select(&selector) {
            // 图片
            let selector = Selector::parse("img").unwrap();
            match element.select(&selector).next() {
                Some(img) => {
                    s.article_pic_url = img.value().attr("src").unwrap().to_string();
                }
                None => {}
            }
            // 商城
            s.article_mall = element.text().collect::<String>().trim().to_string();
            s.article_mall_url = element.value().attr("href").unwrap().to_string();
        }
        // 提取标题和文章链接
        let selector = Selector::parse("h5.feed-ver-title > a").unwrap();
        for element in element.select(&selector) {
            // 提取文章链接
            s.article_url = element.value().attr("href").unwrap().to_string();
            // 提取文章id
            // 创建正则表达式模式，匹配 URL 中的数字
            let re = Regex::new(r"/p/(\d+)/").unwrap();

            if let Some(capture) = re.captures(&s.article_url) {
                if let Some(id) = capture.get(1) {
                    s.article_id = id.as_str().parse::<i64>().unwrap();
                }
            }
            // 提取诸如“白菜党”，“新品尝鲜”，“绝对值”等标签文本
            let selector = Selector::parse("object > a").unwrap();
            if element.select(&selector).count() > 0 {
                match element.select(&selector).last() {
                    Some(a) => {
                        s.zhifa_tag.name = a.inner_html().to_string();
                        s.zhifa_tag.url = a.value().attr("href").unwrap().to_string();
                    }
                    None => {}
                }
                // 创建正则表达式模式，匹配目标文本
                let re = Regex::new(r"\s+(.*)$").unwrap();
                if let Some(capture) = re.captures(element.text().collect::<String>().trim()) {
                    if let Some(text) = capture.get(1) {
                        s.article_title = text.as_str().to_string();
                    }
                    break;
                }
            }
            s.article_title = element.text().collect::<String>().trim().to_string();
        }
        // 提取价格
        let selector = Selector::parse("div.feed-block-ver > div.z-highlight.z-ellipsis").unwrap();
        for element in element.select(&selector) {
            s.article_price = element.text().collect::<String>().trim().to_string();
        }
        // 提取作者
        let selector =
            Selector::parse("div.feed-ver-row-l > span.z-avatar-group > a.z-avatar-pic").unwrap();
        for element in element.select(&selector) {
            match element.value().attr("href") {
                Some(url) => {
                    s.article_avatar_url = url.to_string();
                }
                None => {}
            }
            let selector = Selector::parse("img").unwrap();
            match element.select(&selector).last() {
                Some(a) => {
                    s.article_avatar = a.value().attr("src").unwrap().to_string();
                    s.article_referrals = a.value().attr("alt").unwrap().to_string();
                }
                None => {}
            }
        }
        // 提取日期
        let selector =
            Selector::parse("div.feed-ver-row > div.feed-ver-row-r.feed-ver-date").unwrap();
        for element in element.select(&selector) {
            s.article_date = element.text().collect::<String>().trim().to_string();
        }
        // 提取内容
        let selector = Selector::parse("div.feed-block-ver > div.feed-ver-descripe").unwrap();
        for element in element.select(&selector) {
            let selector = Selector::parse("a").unwrap();
            match element.select(&selector).last() {
                Some(a) => {
                    let link = a.value().attr("href").unwrap().to_string();
                    let text = a.inner_html().to_string();
                    s.article_content = element
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_string()
                        .replace(
                            &text,
                            &format!(
                                "<a href='{}' target='_blank' style='color:#5188a6'>{}</a>",
                                link, text
                            ),
                        );
                    break;
                }
                None => {}
            }
            s.article_content = element.text().collect::<String>().trim().to_string();
        }
        // 提取值的数量
        let selector =
            Selector::parse("span.z-group-data > a.J_zhi_like_fav > span.unvoted-wrap > span")
                .unwrap();
        for element in element.select(&selector) {
            match element.text().collect::<String>().trim().parse::<i64>() {
                Ok(num) => {
                    s.article_rating = num;
                }
                Err(_) => {}
            }
        }
        // 提取评论的数量
        let selector = Selector::parse("a.z-group-data").unwrap();
        for element in element.select(&selector) {
            match element.text().collect::<String>().trim().parse::<i64>() {
                Ok(num) => {
                    s.article_comment = num;
                }
                Err(_) => {}
            }
        }
        // 提取分类
        let selector = Selector::parse(
            "div.feed-link-btn > div.feed-link-btn-inner > a.z-btn, div.feed-link-btn > a.z-btn",
        )
        .unwrap();
        for element in element.select(&selector) {
            match element.value().attr("onclick") {
                Some(text) => {
                    let re = Regex::new(r"gtmAddToCart\((?:.|\n)*?'brand':'(?<brand>.*?)' ?,(?:.|\n)*?'category':'(?<category>.*?)',(?:.|\n)*?\)").unwrap();
                    let mat = re.captures(&text).unwrap();
                    match mat.name("brand") {
                        Some(text) => {
                            s.brand = text.as_str().to_string();
                        }
                        None => {}
                    };
                    match mat.name("category") {
                        Some(text) => {
                            s.cates_str = text.as_str().to_string();
                            s.cates = text.as_str().split("/").map(|x| x.to_string()).collect();
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        let keyword = read_keyword();
        // 对标题进行关键词匹配
        keyword.title.iter().for_each(|x| {
            if s.article_title.contains(&*x) {
                s.article_title = s
                    .article_title
                    .replace(&*x, &format!("<span class=\"text-red-600\">{x}</span>"));
                s.article_border_style = "border: 2px dashed red".to_string();
            }
        });
        // 对标签进行关键词匹配
        keyword.category.iter().for_each(|x| {
            if s.cates.contains(&*x) || s.brand.contains(&*x) {
                s.article_title = s
                    .article_title
                    .replace(&*x, &format!("<span class=\"text-red-600\">{x}</span>"));
                s.article_border_style = "border: 2px dashed red".to_string();
            }
        });
        sl.push(s);
    }
    return Ok(sl);
}

// 获取更多三小时热门榜
pub async fn get_more_three_hour_hot(
    page_num: i32,
) -> Result<SmzdmList, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "faxian.smzdm.com".parse().unwrap());
    headers.insert(
        "accept",
        "application/json, text/javascript, */*; q=0.01"
            .parse()
            .unwrap(),
    );
    headers.insert("accept-language", "zh-CN,zh;q=0.5".parse().unwrap());
    // 读取cookie
    headers.insert(header::COOKIE, read_smzdm_cookie().parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert(
        "referer",
        "https://faxian.smzdm.com/h2s0t0f0c1p1/".parse().unwrap(),
    );
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get(format!(
            "https://faxian.smzdm.com/json_more?filter=h2s0t0f0c1&page={}",
            page_num
        ))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;
    if res == "" {
        return Err("获取三小时热门榜列表失败,遇到机器人检测...".into());
    }
    let v: serde_json::Value = serde_json::from_str(&res)?;
    let mut sl = SmzdmList::new();
    for i in v.as_array().unwrap() {
        let mut s = init_smzdm();
        s.article_id = i["article_id"].as_i64().unwrap();
        s.article_url = i["article_url"].as_str().unwrap().to_string();
        s.article_pic_url = i["article_pic_url"].as_str().unwrap().to_string();
        s.article_pic_style = i["article_pic_style"].as_str().unwrap().to_string();
        s.article_title = i["article_title"].as_str().unwrap().to_string();
        s.article_price = i["article_price"].as_str().unwrap().to_string();
        s.article_referrals = i["article_referrals"].as_str().unwrap().to_string();
        s.article_avatar_url = i["article_avatar_url"].as_str().unwrap().to_string();
        s.article_display_date = i["article_display_date"].as_str().unwrap().to_string();
        s.article_date = i["article_date"].as_str().unwrap().to_string();
        s.article_content = i["article_content"].as_str().unwrap().to_string();
        s.article_yh_type = i["article_yh_type"].as_str().unwrap().to_string();
        s.article_mall = i["article_mall"].as_str().unwrap().to_string();
        s.article_mall_url = i["article_mall_url"].as_str().unwrap().to_string();
        s.article_link = i["article_link"].as_str().unwrap().to_string();
        s.article_rating = i["article_rating"].as_i64().unwrap();
        s.article_avatar = i["article_avatar"].as_str().unwrap().to_string();
        s.article_comment = i["article_comment"].as_i64().unwrap();
        if i["zhifa_tag"].is_array() {
        } else {
            s.zhifa_tag.name = i["zhifa_tag"]["name"].as_str().unwrap().to_string();
            s.zhifa_tag.url = i["zhifa_tag"]["url"].as_str().unwrap().to_string();
        }
        s.brand = i["gtm"]["brand"].as_str().unwrap().to_string();
        s.cates_str = i["gtm"]["cates_str"].as_str().unwrap().to_string();
        s.cates = i["gtm"]["cates_str"]
            .as_str()
            .unwrap()
            .to_string()
            .split("/")
            .map(|x| x.to_string())
            .collect();
        let keyword = read_keyword();
        // 对标题进行关键词匹配
        keyword.title.iter().for_each(|x| {
            if s.article_title.contains(&*x) {
                s.article_title = s
                    .article_title
                    .replace(&*x, &format!("<span class=\"text-red-600\">{x}</span>"));
                s.article_border_style = "border: 2px dashed red".to_string();
            }
        });
        // 对标签进行关键词匹配
        keyword.category.iter().for_each(|x| {
            if s.cates.contains(&*x) || s.brand.contains(&*x) {
                s.article_title = s
                    .article_title
                    .replace(&*x, &format!("<span class=\"text-red-600\">{x}</span>"));
                s.article_border_style = "border: 2px dashed red".to_string();
            }
        });
        sl.push(s);
    }
    Ok(sl)
}

// 读取cookies文件
pub fn read_smzdm_cookie() -> String {
    let file_path = r".\data\smzdm_cookies.txt";
    let _ = fs::create_dir_all(r".\data");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s.trim().to_string()
}

// 读取关键词的json文件
fn read_keyword() -> Keyword {
    let file_path = r".\data\smzdm_keyword.json";
    let _ = fs::create_dir_all(r".\data");
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
                Keyword {
                    title: Vec::new(),
                    category: Vec::new(),
                }
            } else if e.is_data() {
                Keyword {
                    title: Vec::new(),
                    category: Vec::new(),
                }
            } else {
                panic!("{}", e);
            }
        }
    }
}

// 写入关键词的json文件
async fn write_keyword(data: Keyword) {
    let file_path = r".\data\smzdm_keyword.json";
    let _ = tokio::fs::create_dir_all(r".\data").await;
    let file = tokio::fs::OpenOptions::new()
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

// 读取经过关键词筛选的json文件
fn read_filter_item() -> Vec<Smzdm> {
    let file_path = r".\data\smzdm_filter_item.json";
    let _ = fs::create_dir_all(r".\data");
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
                vec![]
            } else if e.is_data() {
                vec![]
            } else {
                panic!("{}", e);
            }
        }
    }
}

// 写入经过关键词筛选的json文件
fn write_filter_item(data: &Vec<Smzdm>) {
    let file_path = r".\data\smzdm_filter_item.json";
    let _ = fs::create_dir_all(r".\data");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .unwrap();
    serde_json::to_writer(file, &data).unwrap();
}

// 定时任务
pub async fn timing_task_smzdm_3hhot(app: tauri::AppHandle) {
    match get_three_hour_hot_list().await {
        Ok(sl) => {
            let a: Vec<Smzdm> = read_filter_item();
            let b: Vec<Smzdm> = sl
                .into_iter()
                .filter(|x| x.article_border_style != "".to_string())
                .collect::<Vec<_>>();
            write_filter_item(&b);
            // 使用 contains 方法会造成因评论数和点值数不同导致重复通知同一个商品
            let c = b
                .into_iter()
                .filter(|x| !a.iter().any(|y| y.article_id == x.article_id))
                .collect::<Vec<_>>();
            println!("c: {:?}", c);
            println!("c.len: {:?}", c.len());
            if c.len() > 0 {
                let _ = tauri::api::notification::Notification::new(
                    &app.config().tauri.bundle.identifier,
                )
                .title("什么值得买热榜")
                .body("您订阅的关键词上榜了！")
                .sound("Default") //系统默认提示音
                .show();
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[tauri::command]
pub async fn smzdm_3hhot() -> Result<Vec<Smzdm>, String> {
    match get_three_hour_hot_list().await {
        Ok(list) => Ok(list),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn smzdm_3hhot_more(pagenum: i32) -> Result<SmzdmList, String> {
    match get_more_three_hour_hot(pagenum).await {
        Ok(list) => Ok(list),
        Err(e) => Err(e.to_string()),
    }
}

// 从文件中读取关键词
#[tauri::command]
pub async fn return_smzdm_keyword() -> Keyword {
    read_keyword()
}

// 改变关键词时写入文件
#[tauri::command]
pub async fn change_smzdm_keyword(params: Keyword) {
    write_keyword(params).await;
}
