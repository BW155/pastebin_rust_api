pub mod access;

use self::access::{Access, get_access};
use construct_api_url;

pub struct Paster {
    developer_key: String,
}

impl Paster {
    pub fn new(developer_key: String) -> Self {
        Paster {
            developer_key: developer_key,
        }
    }

    fn paste(&self, code: &str, access: Access, name: &str, expire_date: &str, format: &str, user_key: &str) -> bool {
        let url = construct_api_url(["api_post.php"]);
        let access = get_access(access);

        let params = [("api_option", "paste"),
                      ("api_user_key", user_key),
                      ("api_paste_private", access)];

    }

    fn send_post_request(url: &str, params: Vec<(&str, &str)>) -> bool {
        let client = reqwest::Client::new().unwrap();
        let res = client.post(url)
            .form(&params)
            .send();
        assert!(resp.status().is_success());
        true
    }
}