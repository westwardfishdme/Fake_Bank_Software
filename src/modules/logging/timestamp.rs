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
    let hour = now.hour();
    let hour = match hour {
        0..10 => format!("0{hour}",),
        _ => format!("{}", hour),
    };
    let minute = now.minute();
    let minute = match minute {
        0..10 => format!("0{minute}",),
        _ => format!("{}", minute),
    };
    let second = now.second();
    let second = match second {
        0..10 => format!("0{second}",),
        _ => format!("{}", second),
    };

    format!(
        "[{} {} {} {}:{}:{}]",
        now.weekday(),
        enumerate_month(now.month()),
        now.day(),
        hour,
        minute,
        second
    )
}
