use cogo::err;

fn main() {
    let e = err!("EOF");
    let e = err!("{:?}","EOF".to_string());
    let e = err!("{},{}","EOF"," and other error");
}