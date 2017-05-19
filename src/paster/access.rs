/// Represents an Enum of possible access types for pastebin.
pub enum Access {
    Public,
    Unlisted,
    Private,
}

/// method for getting the right string for the `Access` enum.
pub fn get_access(access: &Access) -> &str {
    match access {
        &Access::Public => "0",
        &Access::Unlisted => "1",
        &Access::Private => "2",
    }
}
