use std::time::Duration;
use cogo::std::time::time::Time;

fn main() {
    let mut t = Time::now();
    println!("{}", t);
    let js = serde_json::to_string(&t).unwrap();
    println!("{}", js);
    let from_js = serde_json::from_str::<Time>(&js).unwrap();
    assert_eq!(from_js, t);

    t.add(1 * 24 * Duration::from_secs(3600));// add one day
    println!("add one day:{}", t);
    assert_ne!(from_js, t);

    assert_eq!(true, t.before(&Time::now())); //befor

    assert_eq!(true, Time::now().after(&t)); //after

    let formated = t.format("[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
    println!("{}", formated);
}