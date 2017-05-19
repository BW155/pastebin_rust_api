#[derive(Debug)]
pub struct Paste {
    pub key: String,
    pub date: i64,
    pub title: String,
    pub size: i64,
    pub expire_date: i64,
    pub private: i64,
    pub format_short: String,
    pub format_long: String,
    pub url: String,
    pub hits: i64,
}

impl Paste {
    pub fn new(key: String,
               date: String,
               title: String,
               size: String,
               expire_date: String,
               private: String,
               format_short: String,
               format_long: String,
               url: String,
               hits: String)
               -> Self {
        let date = date.parse::<i64>().unwrap_or(0);
        let size = size.parse::<i64>().unwrap_or(0);
        let expire_date = expire_date.parse::<i64>().unwrap_or(0);
        let private = private.parse::<i64>().unwrap_or(0);
        let hits = hits.parse::<i64>().unwrap_or(0);
        Paste {
            key: key,
            date: date,
            title: title,
            size: size,
            expire_date: expire_date,
            private: private,
            format_short: format_short,
            format_long: format_long,
            url: url,
            hits: hits,
        }
    }
}
