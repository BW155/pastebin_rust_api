use reqwest;

use self::access::{Access, get_access};
use self::expiration::{Expiration, get_expiration};
use self::format::{Format, get_format};
use construct_api_url;
use objects::PastebinMessage;

use std::io::Read;
use std::env;

pub mod access;
pub mod expiration;
pub mod format;


use error::{Result, check_for_error};

/// Represents an Paster object for executing pastes.
pub struct Paster {
    developer_key: String,
}

impl Paster {
    /// Constructs a new `Paster` object.
    pub fn new(developer_key: Option<String>) -> Self {
        let developer_key =
            developer_key.or_else(|| env::var("PASTEBIN_DEVELOPER_TOKEN").ok())
                         .expect("You should pass in a token to new or set the PASTEBIN_DEVELOPER_TOKEN env var");
        Paster { developer_key: developer_key }
    }

    /// Pastes your content to pastebin.
    pub fn paste(&self,
                 code: &str,
                 access: &Access,
                 name: &str,
                 expiration: &Expiration,
                 format: &Format,
                 user_key: Option<&str>)
                 -> Result<PastebinMessage> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let access = get_access(access);
        let expiration = get_expiration(expiration);
        let format = get_format(format);
        let dev_key = &self.developer_key;
        let user_key = user_key.unwrap_or("");

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

    /// sends pastebin request, returns `PastebinMessage` on succeed or an `Error` on fail.
    fn send_post_request(&self, url: &str, params: &[(&str, &str)]) -> Result<PastebinMessage> {
        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(url).form(&params).send()?;
        assert!(res.status().is_success());
        let mut result: String = String::new();
        res.read_to_string(&mut result)?;
        Ok(check_for_error(result)?)
    }
}
