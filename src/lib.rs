extern crate reqwest;

pub mod paster;
pub mod objects;
mod error;


fn construct_api_url(path: &[&str]) -> String {
    format!("https://pastebin.com/api/{}", path.join("/"))
}
