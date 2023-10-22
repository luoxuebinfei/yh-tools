use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};
use urlencoding::encode;

use super::smzdm_struct::*;
use super::three_hour_hot::read_smzdm_cookie;

pub async fn search_keyword(keyword: String) -> Result<Vec<Smzdm>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    // 读取cookie
    headers.insert(header::COOKIE, read_smzdm_cookie().parse().unwrap());
    headers.insert("authority", "search.smzdm.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.7".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "none".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.67".parse().unwrap());

    let mut sl = SmzdmList::new();
    let keyword = encode(&keyword).to_string();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get(format!("https://search.smzdm.com/?c=faxian&s={}&order=score&f_c=zhi", keyword))
        .headers(headers)
        .send()
        .await?;
    if res.status() != 200 {
        return Err(format!(r#"{{"status":{}}}"#, res.status().as_u16()).into());
    };
    let document = Html::parse_document(&res.text().await?);
    let selector = Selector::parse("ul#feed-main-list > li").unwrap();
    for element in document.select(&selector).into_iter() {
        let mut s = Smzdm {
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
        // 提取图片url
        let selector = Selector::parse("div.z-feed-img > a > img").unwrap();
        for element in element.select(&selector) {
            s.article_pic_url = format!("https:{}",element.value().attr("src").unwrap().to_string());
        }
        // 提取标题、价格和id
        let selector = Selector::parse("h5.feed-block-title").unwrap();
        for element in element.select(&selector) {
            // 标题
            let selector = Selector::parse("a.feed-nowrap").unwrap();
            for element in element.select(&selector) {
                s.article_title = element.value().attr("title").unwrap().to_string();
                s.article_url = element.value().attr("href").unwrap().to_string();
                // 提取文章id
                // 创建正则表达式模式，匹配 URL 中的数字
                let re = Regex::new(r"/p/(\d+)/").unwrap();
                if let Some(capture) = re.captures(&s.article_url) {
                    if let Some(id) = capture.get(1) {
                        s.article_id = id.as_str().parse::<i64>().unwrap();
                    }
                }
            }
            // 价格
            let selector = Selector::parse("a > div.z-highlight").unwrap();
            for element in element.select(&selector) {
                s.article_price = element.text().collect::<String>().trim().to_string();
            }
        }
        // 提取内容
        let selector = Selector::parse("div.feed-block-descripe-top").unwrap();
        for element in element.select(&selector) {
            s.article_content = element.text().collect::<String>().trim().to_string();
        }
        // 提取值和不值
        let selector = Selector::parse("span.feed-btn-group.price-btn-hover").unwrap();
        for element in element.select(&selector) {
            // 值
            let selector = Selector::parse("span.price-btn-up > span.unvoted-wrap > span").unwrap();
            for element in element.select(&selector) {
                s.article_rating = element
                    .text()
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
                    .unwrap();
            }
            // 不值
            let selector =
                Selector::parse("span.price-btn-down > span.unvoted-wrap > span").unwrap();
            for element in element.select(&selector) {
                s.article_rating_down = element
                    .text()
                    .collect::<String>()
                    .trim()
                    .parse::<i64>()
                    .unwrap();
            }
        }
        // 提取收藏数量
        let selector =
            Selector::parse("span.J_zhi_like_fav.z-group-data.feed-btn-fav > span").unwrap();
        for element in element.select(&selector) {
            s.article_collect = element
                .text()
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .unwrap();
        }
        // 提取评论数量
        let selector = Selector::parse("a.z-group-data.feed-btn-comment").unwrap();
        for element in element.select(&selector) {
            s.article_comment = element
                .text()
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .unwrap();
        }
        // 提取时间和商城
        let selector = Selector::parse("span.feed-block-extras").unwrap();
        for element in element.select(&selector) {
            let re = Regex::new(r"(\d{2}-\d{2} \d{2}:\d{2}|\d{2}:\d{2})").unwrap();
            match re.captures(&element.text().collect::<String>().trim()) {
                Some(mat) => {
                    s.article_date = mat.get(1).unwrap().as_str().to_string();
                }
                None => {}
            }
            let selector = Selector::parse("span").unwrap();
            for element in element.select(&selector) {
                s.article_mall = element.text().collect::<String>().trim().to_string();
            }
        }
        // 提取品牌和分类
        let selector = Selector::parse("div.feed-link-btn-inner > a.z-btn").unwrap();
        for element in element.select(&selector) {
            match element.value().attr("onclick") {
                Some(text) => {
                    // println!("onclick: {}", text);
                    let re = Regex::new(r";gtmAddToCart\(.*?'brand':'(?<brand>.*?)',.*?'category':'(?<category>.*?)',.*?\)").unwrap();
                    match re.captures(&text) {
                        Some(mat) => {
                            match mat.name("brand") {
                                Some(text) => {
                                    s.brand = text.as_str().to_string();
                                }
                                None => {}
                            };
                            match mat.name("category") {
                                Some(text) => {
                                    s.cates_str = text.as_str().to_string();
                                    s.cates =
                                        text.as_str().split("/").map(|x| x.to_string()).collect();
                                }
                                None => {}
                            }
                        },
                        None => {}
                    }
                }
                None => {}
            }
        }
        sl.push(s);
    }
    Ok(sl)
}

#[tauri::command]
pub async fn smzdm_search(keyword: String) -> Result<Vec<Smzdm>, String> {
    match search_keyword(keyword).await{
        Ok(sl) => Ok(sl),
        Err(e) => Err(e.to_string()),
    }
}