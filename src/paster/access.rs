pub enum Access {
    Public,
    Unlisted,
    Private,
}

pub fn get_access(access: &Access) -> &str {
    match access {
        &Access::Public => "0",
        &Access::Unlisted => "1",
        &Access::Private => "2",
    }
}