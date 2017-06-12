 # Pastebin Rust Api

 A Rust wrapper for the [Pastebin Api](https://pastebin.com/api)
 
 ### Installation
 
 Add this to your Cargo.toml `pastebin_rust_api = "0.2.11"` under `[dependencies]`

 ## Getting started

 Simple usage of `Paster`:

 ```Rust
 extern crate pastebin_rust_api;
 use pastebin_rust_api::{Paster, Access, Format, Expiration};

 fn main() {
     // I recommend to put your dev key into an environment variable called `PASTEBIN_DEVELOPER_TOKEN`.
     let parser = Paster::new(Some("<YOUR DEV KEY>".to_owned()));
     let response = parser.paste("<html></html>",
                                 Some(&Access::Private),
                                 Some("TestHtml"),
                                 Some(&Expiration::TenMinutes),
                                 Some(&Format::HTML5),
                                 None);
     if response.is_ok() {
         if let Some(paste) = response.ok() {
             // If everything is OK, you can get the url to your code here.
             println!("{}", paste.content);
         }
     }
 }

 ```
