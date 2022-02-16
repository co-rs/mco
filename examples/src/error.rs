use mco::err;

fn main() {
    let e = err!("EOF");
    let e = err!("{:?}","EOF".to_string());
    let e = err!("error: {},detail: {}","EOF"," detail msgs...");
}