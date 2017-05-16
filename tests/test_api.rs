extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::Paster;
    use pastebin_rust_api::Access;
    use pastebin_rust_api::Expiration;
    use pastebin_rust_api::Format;

    #[test]
    fn test_post() {
        let parser = Paster::new(Some("e27fc5e3a33b4a5e7b509e9d06723fcc".to_owned()));
        let response = parser.paste("<html></html>",
                                    Some(&Access::Private),
                                    Some("TestHtml"),
                                    Some(&Expiration::TenMinutes),
                                    Some(&Format::HTML5),
                                    None);
        assert!(response.is_ok());
        if response.is_ok() {
            if let Some(paste) = response.ok() {
                println!("{}", paste.url);
            }
        }
    }
}
