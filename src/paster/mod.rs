use reqwest;
use treexml::{Document, Element};

use self::access::{Access, get_access};
use self::expiration::{Expiration, get_expiration};
use self::format::{Format, get_format};
use construct_api_url;
use objects::Paste;

use std::io::Read;
use std::env;
use std::fs::File;
use std::path::Path;

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
                           file_path: &Path,
                           access: &Access,
                           name: Option<&str>,
                           expiration: &Expiration,
                           format: &Format,
                           user_key: Option<&str>)
                           -> Result<String> {
        let mut f = File::open(file_path)?;
        let mut code = String::new();
        f.read_to_string(&mut code)?;
        self.paste(&code, access, name, expiration, format, user_key)
    }

    /// Pastes your content to pastebin.
    pub fn paste(&self,
                 code: &str,
                 access: &Access,
                 name: Option<&str>,
                 expiration: &Expiration,
                 format: &Format,
                 user_key: Option<&str>)
                 -> Result<String> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let name = name.unwrap_or("");
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

    /// Logs in to pastebin and returns user_key that can be used for pasting.
    pub fn login(&self,
                 username: Option<String>,
                 password: Option<String>)
                 -> Result<String> {
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

    pub fn get_my_posts(&self, user_key: &str, result_limit: i64) -> Result<Vec<Paste>> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let dev_key: &str = &self.developer_key;
        let result_limit: &str = &format!("{}", result_limit);

        let params = [("api_option", "list"),
                      ("api_user_key", user_key),
                      ("api_result_limit", result_limit),
                      ("api_dev_key", dev_key)];
        let mut xml: String = self.send_post_request(&url, &params)?;
        xml.insert_str(0, "<root>");
        xml.push_str("</root>");
        let doc = Document::parse(xml.as_bytes()).unwrap_or(Document::new());
        let root = doc.root.unwrap();
        let pastes = self.process_xml_pastes(root);
        Ok(pastes)
    }

    /// Returns Vector of Paste objects
    pub fn get_trending_posts(&self) -> Result<Vec<Paste>> {
        let path = ["api_post.php"];
        let url = construct_api_url(&path);
        let dev_key: &str = &self.developer_key;

        let params = [("api_option", "trends"),
                      ("api_dev_key", dev_key)];
        let mut xml: String = self.send_post_request(&url, &params)
                                  .unwrap_or(String::new());
        xml.insert_str(0, "<root>");
        xml.push_str("</root>");
        let doc = Document::parse(xml.as_bytes()).unwrap_or(Document::new());
        let root = doc.root.unwrap();
        let pastes = self.process_xml_pastes(root);
        Ok(pastes)
    }

    fn process_xml_pastes(&self, root: Element) -> Vec<Paste> {
        let mut pastes = Vec::new();
        for i in root.children {
            let key = i.find_child(|e| e.name == "paste_key")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let date = i.find_child(|e| e.name == "paste_date")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let title = i.find_child(|e| e.name == "paste_date")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let size = i.find_child(|e| e.name == "paste_size")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let expire = i.find_child(|e| e.name == "paste_expire_date")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let private = i.find_child(|e| e.name == "paste_private")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let format_short = i.find_child(|e| e.name == "paste_format_short")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let format_long = i.find_child(|e| e.name == "paste_format_long")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let url = i.find_child(|e| e.name == "paste_url")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let hits = i.find_child(|e| e.name == "paste_hits")
                .map(|k| k.clone())
                .map(|k| k.text.unwrap_or(String::new()))
                .unwrap();
            let paste = Paste::new(key,
                                   date,
                                   title,
                                   size,
                                   expire,
                                   private,
                                   format_short,
                                   format_long,
                                   url,
                                   hits);
            pastes.push(paste);
        }
        pastes
    }

    /// sends pastebin request, returns `String` on succeed or an `Error` on fail.
    fn send_post_request(&self, url: &str, params: &[(&str, &str)]) -> Result<String> {
        let client = reqwest::Client::new().unwrap();
        let mut res = client.post(url).form(&params).send()?;
        assert!(res.status().is_success());
        let mut result: String = String::new();
        res.read_to_string(&mut result)?;
        Ok(check_for_error(result)?)
    }
}
