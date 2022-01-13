use cogo::std::time::time::Time;

fn main() {
    let t = Time::now();
    println!("{}", t);
    let js = serde_json::to_string(&t).unwrap();
    println!("{}", js);
    let from_js = serde_json::from_str::<Time>(&js).unwrap();
    assert_eq!(from_js, t);
}