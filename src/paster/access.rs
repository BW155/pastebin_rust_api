pub enum Access {
    PUBLIC,
    UNLISTED,
    PRIVATE,
}

pub fn get_access(access: &Access) -> &str {
    match access {
        &Access::PUBLIC => "0",
        &Access::UNLISTED => "1",
        &Access::PRIVATE => "2",
    }
}