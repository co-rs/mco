use once_cell::sync::Lazy;

pub const longDayNames: Lazy<Vec<&str>> = Lazy::new(|| {
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

pub const shortDayNames: Lazy<Vec<&str>> = Lazy::new(|| {
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

pub const shortMonthNames: Lazy<Vec<&str>> = Lazy::new(|| {
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

pub const longMonthNames: Lazy<Vec<&str>> = Lazy::new(|| {
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

