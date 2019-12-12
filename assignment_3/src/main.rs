
// Question.1
/*
fn adobe_photoshop<T:FnOnce()> (message:T) {
    message();
}

fn main() {
    let trial = ||println!("You trial has been expired");
    adobe_photoshop(trial);
}
*/


// Question.2
/*
fn power_four<T:FnMut()>(mut number:T) {
    number();
    number();
}
fn main() {
    let mut number = 4;
    println!("Number = {}",number);
    let square = || number = number * number ; 
    power_four(square);
    println!("Power four of a number = {}",number);
}
*/

// Question.3
/*
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn ( || {
            for k in 1..10 {
                println!("From Sapwn Thread {}",k);
                thread::sleep(Duration::from_millis(1));
            }
        }
    );
    for j in 1..8 {
        println!("From Main Thread {}",j);
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();
}
*/







