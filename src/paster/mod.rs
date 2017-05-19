use reqwest;

use self::access::{Access, get_access};
use self::expiration::{Expiration, get_expiration};
use self::format::{Format, get_format};
use construct_api_url;
use objects::PastebinMessage;

use std::io::Read;
use std::env;
use std::fs::File;

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
            developer_key
                .or_else(|| env::var("PASTEBIN_DEVELOPER_TOKEN").ok())
                .expect("Environment variable 'PASTEBIN_DEVELOPER_TOKEN' not found");
        Paster { developer_key: developer_key }
    }

    /// Pastes the content of your file to pastebin.
    pub fn paste_from_file(&self,
                           file_path: &str,
                           access: Option<&Access>,
                           name: Option<&str>,
                           expiration: Option<&Expiration>,
                           format: Option<&Format>,
                           user_key: Option<&str>)
                           -> Result<PastebinMessage> {
        let mut f = File::open(file_path)?;
        let mut code = String::new();
        f.read_to_string(&mut code)?;
        self.paste(&code, access, name, expiration, format, user_key)
    }

    /// Pastes your content to pastebin.
    pub fn paste(&self,
                 code: &str,
                 access: Option<&Access>,
                 name: Option<&str>,
                 expiration: Option<&Expiration>,
                 format: Option<&Format>,
                 user_key: Option<&str>)
                 -> Result<PastebinMessage> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let name = name.unwrap_or("");
        let access = access.map(|a| get_access(a)).unwrap_or("");
        let expiration = expiration.map(|e| get_expiration(e)).unwrap_or("");
        let format = format.map(|f| get_format(f)).unwrap_or("");
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

    /// Logs in to pastebin and returns user_key that can be used for pasting.
    pub fn login(&self,
                 username: Option<String>,
                 password: Option<String>)
                 -> Result<PastebinMessage> {
        let path = ["api_login.php"];
        let url = construct_api_url(&path);
        let dev_key: &str = &self.developer_key;
        let username =
            &username
                 .or_else(|| env::var("PASTEBIN_USER_NAME").ok())
                 .expect("You should pass in a username or set the PASTEBIN_USER_NAME env var");
        let password =
            &password
                 .or_else(|| env::var("PASTEBIN_USER_PASSWORD").ok())
                 .expect("You should pass in a password or set the PASTEBIN_USER_PASSWORD env var");


        let params = [("api_dev_key", dev_key),
                      ("api_user_name", username),
                      ("api_user_password", password)];
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
