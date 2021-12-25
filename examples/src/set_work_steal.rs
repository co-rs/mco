use cogo::go;

fn main(){
    //The task is submitted to the local queue first.
    // Only when there are no tasks in the local queue,
    // the task is stolen from other queues. For the most part, they don't
    cogo::config().set_work_steal(false);

    // Commit to the global queue first.
    cogo::config().set_work_steal(true);

    go!(||{
       println!("");
    });
}