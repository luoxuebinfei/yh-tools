use serde::{Deserialize, Serialize};

pub type SmzdmList = Vec<Smzdm>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Smzdm {
    pub article_id: i64,              // 文章id
    pub article_url: String,          // 文章url
    pub article_pic_url: String,      // 图片url
    pub article_pic_style: String,    // 图片样式
    pub article_title: String,        // 文章标题
    pub article_price: String,        // 价格
    pub article_referrals: String,    // 作者
    pub article_avatar: String,       // 作者头像url
    pub article_avatar_url: String,   // 作者主页url
    pub article_display_date: String, // 发布时间
    pub article_date: String,         // 发布时间
    pub article_content: String,      // 内容
    pub article_yh_type: String,      // 优惠类型
    pub article_mall: String,         // 商城
    pub article_mall_url: String,     // 商城url
    pub article_link: String,         // 链接
    pub article_rating: i64,          // “值”的数量
    pub article_rating_down: i64,     // “不值”的数量
    pub article_collect: i64,         // 收藏数量
    pub article_comment: i64,         // 评论数量
    pub zhifa_tag: ZhifaTag,          // 诸如“白菜党”，“新品尝鲜”，“绝对值”等标签
    pub article_border_style: String, // 符合关键词的边框样式
    pub cates_str: String,            // 分类字符串
    pub cates: Vec<String>,           // 分类列表
    pub brand: String,                // 品牌
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ZhifaTag {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize,Serialize)]
pub struct Keyword{
    pub title: Vec<String>,
    pub category: Vec<String>,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Monitor{
    pub title: String,
    pub url: String,
    pub content: String,
    pub is_update: bool,
    pub is_expired: bool, // 是否过期
}