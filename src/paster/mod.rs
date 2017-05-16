use reqwest;

use self::access::{Access, get_access};
use self::expiration::{Expiration, get_expiration};
use construct_api_url;
use objects::PastebinMessage;

use std::io::Read;
use std::error::Error;
use std::env;

pub mod access;
pub mod expiration;


use error::{Result};

pub struct Paster {
    developer_key: String,
}

impl Paster {
    pub fn new(developer_key: Option<String>) -> Self {
        let developer_key =
            developer_key.or_else(|| env::var("PASTEBIN_DEVELOPER_TOKEN").ok())
                         .expect("You should pass in a token to new or set the PASTEBIN_DEVELOPER_TOKEN env var");
        Paster {
            developer_key: developer_key,
        }
    }

    pub fn paste(&self, code: &str, access: &Access, name: &str, expiration: &Expiration, format: &str, user_key: &str) -> Result<PastebinMessage> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let access = get_access(access);
        let expiration = get_expiration(expiration);
        let dev_key = &self.developer_key;

        let params = [("api_option", "paste"),
                      ("api_user_key", user_key),
                      ("api_paste_private", access),
                      ("api_paste_name", name),
                      ("api_expire_date", expiration),
                      ("api_paste_format", format),
                      ("api_dev_key", dev_key),
                      ("api_paste_code", code)];

        self.send_post_request(&url, &params)
    }

    fn send_post_request(&self, url: &str, params: &[(&str, &str)]) -> Result<PastebinMessage> {
        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(url).form(&params).send()?;
        assert!(res.status().is_success());
        let mut result = String::new();
        res.read_to_string(&mut result);
        let message = PastebinMessage { url: result };
        Ok(message)
    }
}
