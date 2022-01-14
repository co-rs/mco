use once_cell::sync::Lazy;

pub static longDayNames: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ]
});

pub static shortDayNames: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Sun",
        "Mon",
        "Tue",
        "Wed",
        "Thu",
        "Fri",
        "Sat",
    ]
});

pub static shortMonthNames: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    ]
});

pub static longMonthNames: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December", ]
});

