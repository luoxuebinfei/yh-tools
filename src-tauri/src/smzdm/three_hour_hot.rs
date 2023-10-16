use regex::Regex;
use reqwest::header;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

pub type SmzdmList = Vec<Smzdm>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Smzdm {
    article_id: i64,              // 文章id
    article_url: String,          // 文章url
    article_pic_url: String,      // 图片url
    article_pic_style: String,    // 图片样式
    article_title: String,        // 文章标题
    article_price: String,        // 价格
    article_referrals: String,    // 作者
    article_avatar: String,       // 作者头像url
    article_avatar_url: String,   // 作者主页url
    article_display_date: String, // 发布时间
    article_date: String,         // 发布时间
    article_content: String,      // 内容
    article_yh_type: String,      // 优惠类型
    article_mall: String,         // 商城
    article_mall_url: String,     // 商城url
    article_link: String,         // 链接
    article_rating: i64,          // “值”的数量
    article_comment: i64,         // 评论数量
    zhifa_tag: ZhifaTag,          // 诸如“白菜党”，“新品尝鲜”，“绝对值”等标签
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ZhifaTag {
    name: String,
    url: String,
}

// 获取三小时热门榜列表
async fn get_three_hour_hot_list() -> Result<Vec<Smzdm>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.smzdm.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.7".parse().unwrap());
    headers.insert("cache-control", "max-age=0".parse().unwrap());
    // headers.insert(header::COOKIE, "__ckguid=OQP2FFq25pftU2scxpetkx5; device_id=25333318211695529551167690f867efc54b19ccf922727ec10a2ad6df; homepage_sug=d; r_sort_type=score; footer_floating_layer=0; _zdmA.uid=ZDMA.oBttSYH01.1696757931.2419200; ad_date=8; bannerCounter=%5B%7B%22number%22%3A0%2C%22surplus%22%3A1%7D%2C%7B%22number%22%3A0%2C%22surplus%22%3A1%7D%2C%7B%22number%22%3A0%2C%22surplus%22%3A1%7D%2C%7B%22number%22%3A0%2C%22surplus%22%3A1%7D%2C%7B%22number%22%3A0%2C%22surplus%22%3A1%7D%5D; ad_json_feed=%7B%7D; smzdm_user_view=51981C4817B08B5D89809E728241E413; smzdm_user_source=E9E14452A6057E99E47AF0ADB9866005; s_his=%E4%B9%90%E4%BA%8B%2C%E5%8F%AF%E4%B9%90%2C%E6%8A%BD%E7%BA%B8; ss_ab=ss66; ssmx_ab=mxss59; smzdm_ec=06; smzdm_ea=01".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("referer", "https://search.smzdm.com/".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36".parse().unwrap());

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
    let document = Html::parse_document(&res);
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
            article_comment: 0,
            zhifa_tag: ZhifaTag {
                name: String::new(),
                url: String::new(),
            },
        };
        // 提取文章图片和商城
        let selector = Selector::parse("div.feed-ver-pic > a").unwrap();
        for element in element.select(&selector) {
            // 图片
            let selector = Selector::parse("img").unwrap();
            match element.select(&selector).next(){
                Some(img) => {
                    s.article_pic_url = img.value().attr("src").unwrap().to_string();
                },
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
                    s.article_content = element.text().collect::<String>().trim().to_string().replace(&text, &format!("<a href='{}' target='_blank' style='color:#5188a6'>{}</a>", link, text));
                    break;
                }
                None => {}
            }
            s.article_content = element.text().collect::<String>().trim().to_string();
        }
        // 提取值的数量
        let selector = Selector::parse("span.z-group-data > a > span.unvoted-wrap > span").unwrap();
        for element in element.select(&selector) {
            s.article_rating = element
                .text()
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .unwrap();
        }
        // 提取评论的数量
        let selector = Selector::parse("a.z-group-data").unwrap();
        for element in element.select(&selector) {
            s.article_comment = element
                .text()
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .unwrap();
        }
        sl.push(s);
    }
    return Ok(sl);
}

// 获取更多三小时热门榜
pub async fn get_more_three_hour_hot(page_num: i32) -> Result<SmzdmList, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "faxian.smzdm.com".parse().unwrap());
    headers.insert("accept", "application/json, text/javascript, */*; q=0.01".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.5".parse().unwrap());
    // headers.insert(header::COOKIE, "__ckguid=OQP2FFq25pftU2scxpetkx5; device_id=25333318211695529551167690f867efc54b19ccf922727ec10a2ad6df; homepage_sug=d; r_sort_type=score; s_his=%E4%B9%90%E4%BA%8B%2C%E5%8F%AF%E4%B9%90%2C%E6%8A%BD%E7%BA%B8; ss_ab=ss66; ssmx_ab=mxss59; smzdm_ec=06; smzdm_ea=02".parse().unwrap());
    headers.insert("dnt", "1".parse().unwrap());
    headers.insert("referer", "https://faxian.smzdm.com/h2s0t0f0c1p1/".parse().unwrap());
    headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-gpc", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.get(format!("https://faxian.smzdm.com/json_more?filter=h2s0t0f0c1&page={}", page_num))
        .headers(headers)
        .send()
        .await?
        .text().await?;
    let v :serde_json::Value= serde_json::from_str(&res)?;
    let mut sl = SmzdmList::new();
    for i in v.as_array().unwrap(){
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
            article_comment: 0,
            zhifa_tag: ZhifaTag {
                name: String::new(),
                url: String::new(),
            },
        };
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
        s.article_avatar =  i["article_avatar"].as_str().unwrap().to_string();
        s.article_comment = i["article_comment"].as_i64().unwrap();
        if i["zhifa_tag"].is_array(){
        }else {
            s.zhifa_tag.name = i["zhifa_tag"]["name"].as_str().unwrap().to_string();
            s.zhifa_tag.url = i["zhifa_tag"]["url"].as_str().unwrap().to_string();
        }
        sl.push(s);
    }
    Ok(sl)
} 

#[tauri::command]
pub async fn smzdm_3hhot() -> Result<Vec<Smzdm>, String> {
    match get_three_hour_hot_list().await{
        Ok(list) => {
            Ok(list)
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn smzdm_3hhot_more(pagenum: i32) -> Result<SmzdmList, String> {
    match get_more_three_hour_hot(pagenum).await{
        Ok(list) => {
            Ok(list)
        },
        Err(e) => {
            Err(e.to_string())
        }
    }
}