use cogo::std::channel::unbounded;

fn main() {
    let (s,r) = unbounded();
    s.send(1);
    let rv=r.recv().unwrap();
    println!("{}",rv);
}