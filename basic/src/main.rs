use std::{io::Error, thread, time::Duration};

async fn foo(id: i32) {
    for i in 0..3 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

async fn sub() {
    foo(10).await;
    foo(20).await;
    foo(30).await;
}

fn main() {
    println!("process start");
    futures::executor::block_on(sub());
    println!("process end");
}
