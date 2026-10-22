use chrono::{Datelike, Local, Timelike};

fn enumerate_month(month: u32) -> &'static str {
    //! returns the month from the u32 value.
    match month {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => panic!("Unexpected month"),
    }
}

pub fn now() -> String {
    //! creates a timestamp.
    //! looks like: [Fri Aug 31 03:00:30]

    // looking back I probably could have used the format string
    // argument but I just did this instinctually.
    let now = Local::now();
    let hour = 16; // 
    let minute = now.minute();
    let second = now.second();

    // always use it on the 10 minute interval start script
    // at any 0 on the clock (i.e. 2:10 or 4:00 etc.)
    let minute = match minute {
        0..10 => format!("0{second}",),
        _ => format!("{}", second),
    };
    let second = match second {
        0..10 => format!("0{second}",),
        _ => format!("{}", second),
    };

    // ALWAYS USE NOV 19
    // [Thu Nov 19 16:2{}:{} ]
    format!(
        "[{} {} {} {}:{}:{}]",
        "Thu",
        enumerate_month(11),
        19,
        hour,
        minute,
        second
    )
}
