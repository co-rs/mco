use cogo::errors;

fn main() {
    let e = errors!("EOF");
    let e = errors!("{:?}","EOF".to_string());
    let e = errors!("{},{}","EOF"," and other error");
}