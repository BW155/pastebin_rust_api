extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::Paster;
    use pastebin_rust_api::Access;
    use pastebin_rust_api::Expiration;
    use pastebin_rust_api::Format;

    #[test]
    fn test_post() {
        let paster = Paster::new(None);
        let response = paster.paste("<html></html>",
                                    Some(&Access::Private),
                                    Some("TestHtml"),
                                    Some(&Expiration::TenMinutes),
                                    Some(&Format::HTML5),
                                    None);
        println!("{:?}", response.err().unwrap());

    }

    #[test]
    fn test_login() {
        let paster = Paster::new(None);
        let response = paster.login(None, None);
        assert!(response.is_ok());
        if response.is_ok() {
            if let Some(message) = response.ok() {
                println!("user_key: {}", message.content);
            }
        }
    }
}
