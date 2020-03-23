use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    let stdin = stdin();
    let mut input = String::new();

    // Intro message
    println!("\nPlease provide the data requested below to setup rusTIG environment :)");
    println!("GitHub username:");
    stdout().flush().unwrap();

    // Read user input and assign its value into rustig.username
    stdin.read_line(&mut input);
    Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("rustig.username")
            .arg(input.replace('\n', ""))
            .output()
            .expect("");
    input.clear();
    
    println!("\nGitHub access token:");
    stdout().flush().unwrap();

    // Read user input and assign its value into rustig.token
    stdin.read_line(&mut input);
    Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("rustig.token")
            .arg(input.replace('\n', ""))
            .output()
            .expect("");

    println!("\nSetup completed! Now you can use rusTIG advanced commands :)");
}