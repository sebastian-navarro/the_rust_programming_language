use std::{thread, time::Duration};




fn main(){
    let handle = thread::spawn( ||  {
        for i in 1..10 {
            println!("THREAD NUMBER {} from the thread spawn" , i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("NORMAL NUMBER {} from the thread spawn" , i);
    }

    handle.join().unwrap();
}