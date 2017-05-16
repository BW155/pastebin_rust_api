extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::Paster;
    use pastebin_rust_api::Access;
    use pastebin_rust_api::Expiration;

    #[test]
    fn test_post() {
        let parser = Paster::new(Some("e27fc5e3a33b4a5e7b509e9d06723fcc".to_owned()));
        let response = parser.paste("<html></html>", &Access::Unlisted, "TestHtml", &Expiration::TenMinutes, "html5", "");
        assert!(response.is_ok());
        println!("url: {}", response.ok().unwrap().url);
    }
}