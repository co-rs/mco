use cogo::std::blocking::spawn_blocking;

fn main(){
    let v = spawn_blocking(|| {
        return 1;
    });
    assert_eq!(v.unwrap(), 1);
}