use std::io::{stdin, stdout, BufRead, Write};

fn main() {
    println!("\nPlease provide the data requested below to setup rusTIG environment :)");
    print!("GitHub username: ");
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();

    println!("Buffer is {:?}", line1);
    // println!("Handle is {:?}", handle);
}