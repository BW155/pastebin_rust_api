
//! # Pastebin Rust Api
//!
//! Wrapper for the [Pastebin Api](https://pastebin.com/api)
//!
//! ## Getting started
//!
//! Simple usage of `Paster`:
//!
//! ```
//! // I recommend to put your dev key into an environment variable called `PASTEBIN_DEVELOPER_TOKEN`.
//! let parser = Paster::new(Some("<YOUR DEV KEY>".to_owned()));
//! let response = parser.paste("<html></html>",
//!                             Some(&Access::Private),
//!                             Some("TestHtml"),
//!                             Some(&Expiration::TenMinutes),
//!                             Some(&Format::HTML5),
//!                             None);
//! if response.is_ok() {
//!     if let Some(paste) = response.ok() {
//!         // If everything is OK, you can get the url to your code here.
//!         println!("{}", paste.url);
//!     }
//! }
//!
//! ```
//!

extern crate reqwest;

pub use self::paster::Paster;
pub use self::paster::expiration::Expiration;
pub use self::paster::access::Access;
pub use self::paster::format::Format;
pub use self::error::Error;

mod paster;
mod objects;
mod error;


/// method for constructing the api url for pastebin.
fn construct_api_url(path: &[&str]) -> String {
    format!("https://pastebin.com/api/{}", path.join("/"))
}
