#![allow(non_snake_case)]

extern crate reqwest;

use std::io;
use std::fs::File;
use std::process::Command;

fn main() {
    let mut resp = reqwest::get("https://github.com/FireWall-e/rusTIG/raw/master/target/release/git-crepo.exe").expect("request failed");
    let mut out = File::create("git-crepo.exe").expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");

    // Get git executives path
    let output = Command::new("git")
                         .arg("--exec-path")
                         .output()
                         .expect("Error while executing 'git --exec-path' command!");

    println!("output is {:?}", output);
    let git_executives_path = output.stdout;
}