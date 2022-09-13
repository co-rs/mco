use mco::{defer, spawn_blocking};

fn main() {
    let v = spawn_blocking!(|| {
        return 1;
    });
    match v {
        Ok(v) => {
            println!("{}", v);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
