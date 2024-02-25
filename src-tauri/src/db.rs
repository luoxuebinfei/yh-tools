use std::fs;
use std::path::Path;

// 检查数据库文件是否存在，如果不存在则创建一个。
pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

// 创建数据库文件.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // 如果父目录不存在，则创建父目录。
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // 创建数据库文件.
    let conn = rusqlite::Connection::open(db_path).unwrap();
    // 创建xianbaoku表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS xianbaoku (
            cateid TEXT NOT NULL,
            comments INTEGER NOT NULL,
            content TEXT NOT NULL,
            catename TEXT NOT NULL,
            datetime TEXT NOT NULL,
            id INTEGER PRIMARY KEY,
            louzhu TEXT NOT NULL,
            louzhuregtime TEXT,
            shijianchuo INTEGER NOT NULL,
            shorttime TEXT NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            yuanurl TEXT NOT NULL
        );
        ",
        (),
    ).unwrap();
    // 创建smzdm表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS smzdm (
        article_id INTEGER PRIMARY KEY,
        article_url TEXT NOT NULL,
        article_pic_url TEXT NOT NULL,
        article_pic_style TEXT NOT NULL,
        article_title TEXT NOT NULL,
        article_price TEXT NOT NULL,
        article_referrals TEXT NOT NULL,
        article_avatar TEXT NOT NULL,
        article_avatar_url TEXT NOT NULL,
        article_display_date TEXT NOT NULL,
        article_date TEXT NOT NULL,
        article_content TEXT NOT NULL,
        article_yh_type TEXT NOT NULL,
        article_mall TEXT NOT NULL,
        article_mall_url TEXT NOT NULL,
        article_link TEXT NOT NULL,
        article_rating INTEGER NOT NULL,
        article_rating_down INTEGER NOT NULL,
        article_collect INTEGER NOT NULL,
        article_comment INTEGER NOT NULL,
        zhifa_tag TEXT NOT NULL,
        article_border_style TEXT NOT NULL,
        cates_str TEXT NOT NULL,
        brand TEXT NOT NULL
    );
    ",
        (),
    ).unwrap();
}

// 检查数据库文件是否存在。
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// 获取数据库文件应所在的路径。
pub fn get_db_path() -> String {
    "./data/database.db".to_string()
}
