use chrono::{Datelike, Local, Timelike};

fn enumerate_month(month: u32) -> &'static str {
    //! returns the month from the u32 value.
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Unexpected month"),
    }
}

pub fn now() -> String {
    //! creates a timestamp.
    //! looks like: [Fri Aug 31 03:00:30]
    let now = Local::now();

    format!(
        "[{} {} {} {}:{}:{}]",
        now.weekday(),
        enumerate_month(now.month()),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    )
}
