use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi numbrer {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi numbrer {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    } 
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move ||{ //is necesary to move v to thread
        println!("Here´s a vector: {:?}", v);
    });
    //drop(v); can´t use drop, the owner of v is the thread
    handle.join().unwrap();
}
