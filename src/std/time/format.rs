use crate::std::lazy::sync::Lazy;

pub static LONG_DAY_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
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

pub static SHORT_DAY_NAMES: Lazy<Vec<&str>> =
    Lazy::new(|| vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"]);

pub static SHORT_MONTH_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
});

pub static LONG_MONTH_NAMES: Lazy<Vec<&str>> = Lazy::new(|| {
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
        "December",
    ]
});
