extern crate pastebin_rust_api;

#[cfg(test)]
mod tests {
    use pastebin_rust_api::{Paster, Access, Expiration, Format};

    #[test]
    fn test_post() {
        let paster = Paster::new(None);
        let response = paster.paste("<html></html>",
                                    Some(&Access::Private),
                                    Some("TestHtml"),
                                    Some(&Expiration::TenMinutes),
                                    Some(&Format::HTML5),
                                    None);
        assert!(response.is_ok());
        if let Some(message) = response.ok() {
            println!("URL: {}", message.content);
        }
    }

    #[test]
    fn test_login() {
        let paster = Paster::new(None);
        let response = paster.login(None, None);
        assert!(response.is_ok());
        if let Some(user_key) = response.ok() {
            println!("user_key: {}", user_key.content);
        }
    }

    #[test]
    fn test_login_post() {
        let paster = Paster::new(None);
        let user_key_response = paster.login(None, None);
        assert!(user_key_response.is_ok());
        if let Some(user_key) = user_key_response.ok() {
            let url_response = paster.paste("<html></html>",
                                        Some(&Access::Private),
                                        Some("TestHtml"),
                                        Some(&Expiration::TenMinutes),
                                        Some(&Format::HTML5),
                                        Some(&user_key.content));
            assert!(url_response.is_ok());
            if let Some(message) = url_response.ok() {
                println!("URL: {}", message.content);
            }
        }
    }

    #[test]
    fn test_file_post() {
        let paster = Paster::new(None);
        let response = paster.paste_from_file("test.html",
                                              Some(&Access::Private),
                                              Some("TestHtml"),
                                              Some(&Expiration::TenMinutes),
                                              Some(&Format::HTML5),
                                              None);
        assert!(response.is_ok());
        if let Some(message) = response.ok() {
            println!("URL: {}", message.content);
        }
    }
}
