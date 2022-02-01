use std::time::Duration;
use cogo::std::time::time::Time;
use cogo::std::time::time;

fn main() {
    let mut t = Time::now();
    println!("{}", t);
    println!("{:?}", t);
    println!("{}", t.unix());
    println!("{}", t.unix_nano());

    //json
    let js = serde_json::json!(&t).to_string();
    println!("{}", js);
    let from_js = serde_json::from_str::<Time>(&js).unwrap();
    assert_eq!(from_js, t);

    //add 1 day
    t.add(1 * 24 * Duration::from_secs(3600));
    println!("added 1 day:{}", t);
    assert_ne!(from_js, t);

    //is before?
    assert_eq!(true, t.before(&Time::now()));

    //is after?
    assert_eq!(true, Time::now().after(&t));

    //parse from str
    let parsed = Time::parse(time::RFC3339Nano, &t.to_string()).unwrap();
    assert_eq!(t, parsed);

    //format time to str
    let formatted = t.format(time::RFC3339);
    println!("formatted: {}", formatted);

    let formatted = t.format(time::RFC3339Nano);
    println!("formatted: {}", formatted);

    let formatted = t.format("[year]-[month] [ordinal] [weekday] [week_number]-[day] [hour]:[minute] [period]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
    println!("formatted: {}", formatted);

    let formatted = t.format(time::RFC1123);
    println!("formatted: {}", formatted);

    let formatted = t.utc();
    println!("to utc: {}", formatted);
    assert_eq!(t, formatted.local());

    let formatted = t.local();
    println!("to local: {}", formatted);
    assert_eq!(t, formatted);

    println!("default(): {}", Time::default());
    assert_eq!(true, Time::default().is_zero());
}