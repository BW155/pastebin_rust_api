pub enum Expiration {
    Never,
    TenMinutes,
    OneHour,
    OneDay,
    OneWeek,
    TwoWeeks,
    OneMonth,
}

pub fn get_expiration(expiration: &Expiration) -> &str {
    match expiration {
        &Expiration::Never => "N",
        &Expiration::TenMinutes => "10M",
        &Expiration::OneHour => "1H",
        &Expiration::OneDay => "1D",
        &Expiration::OneWeek => "1W",
        &Expiration::TwoWeeks => "2W",
        &Expiration::OneMonth => "1M",
    }
}