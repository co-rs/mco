use mco::{co, yield_now};


fn main() {
    mco::get_runtime().spawn(async{
       println!("hay");
       tokio::task::yield_now().await;
    });
    mco::get_runtime().block_on(async {

    });
    mco::get_runtime().block_on(async{

    });
}