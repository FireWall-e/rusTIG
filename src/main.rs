#![allow(non_snake_case)]

use std::process::Command;

fn main() {
    let output = Command::new("git")
                         .arg("--exec-path")
                         .output()
                         .expect("Error while executing 'git --exec-path' command!");

    println!("output is {:?}", output);
    let gitExecutivesPath = output.stdout;
}
