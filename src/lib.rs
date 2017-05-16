extern crate reqwest;

pub use self::paster::Paster;
pub use self::objects::PastebinMessage;
pub use self::paster::expiration::Expiration;
pub use self::paster::access::Access;
pub use self::paster::format::Format;
pub use self::error::Error;

mod paster;
pub mod objects;
mod error;


fn construct_api_url(path: &[&str]) -> String {
    format!("https://pastebin.com/api/{}", path.join("/"))
}
