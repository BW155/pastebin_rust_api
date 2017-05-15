extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::paster::Paster;
    use pastebin_rust_api::paster::access::Access;

    #[test]
    fn test_post() {
        let parser = Paster::new(Some("e27fc5e3a33b4a5e7b509e9d06723fcc".to_owned()));
        let response = parser.paste("<html></html>", &Access::UNLISTED, "TestHtml", "1M", "html", "");
        assert!(response.is_ok());
        println!("url: {}", response.ok().unwrap().url);
    }
}