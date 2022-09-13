use mco::std::time::time::Time;
use mco::std::time::{time, UtcOffset};
use std::time::Duration;

fn main() {
    let mut now = Time::now();
    println!("{}", now);
    println!("{:?}", now);
    println!("{}", now.unix());
    println!("{}", now.unix_nano());

    //json
    let js = serde_json::json!(&now).to_string();
    println!("{}", js);
    let from_js = serde_json::from_str::<Time>(&js).unwrap();
    assert_eq!(from_js, now);

    //add 1 day
    let mut add = now.clone().add(1 * 24 * Duration::from_secs(3600));
    println!("add 1 day:{}", add);

    //sub 1 day
    let mut sub = now.clone().sub(1 * 24 * Duration::from_secs(3600));
    println!("sub 1 day:{}", sub);

    //is before?
    assert_eq!(true, now.before(&Time::now()));

    //is after?
    assert_eq!(true, Time::now().after(&now));

    //parse from str
    let parsed = Time::parse(time::RFC3339_NANO, &now.to_string()).unwrap();
    assert_eq!(now, parsed);

    //format time to str
    let formatted = now.format(time::RFC3339);
    println!("formatted: {}", formatted);

    let formatted = now.format(time::RFC3339_NANO);
    println!("formatted: {}", formatted);

    let formatted = now.format("[year]-[month] [ordinal] [weekday] [week_number]-[day] [hour]:[minute] [period]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
    println!("formatted: {}", formatted);

    let formatted = now.format(time::RFC1123);
    println!("formatted: {}", formatted);

    let formatted = now.utc();
    println!("to utc: {}", formatted);
    assert_eq!(now, formatted.local());

    let formatted = now.local();
    println!("to local: {}", formatted);
    assert_eq!(now, formatted);

    println!("default(): {}", Time::default());
    assert_eq!(true, Time::default().is_zero());

    //to offset
    Time::now().to_offset(UtcOffset::UTC);
}
