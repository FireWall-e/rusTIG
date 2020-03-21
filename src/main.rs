#![allow(non_snake_case)]

extern crate reqwest;

use std::io;
use std::fs::File;
use std::process::Command;
use std::str;
use std::fs::read_dir;
use std::fs::set_permissions;

fn print_type_of<T>(_: &T) {
    println!("Type is {}", std::any::type_name::<T>())
}

fn main() {
    // Get compiled executable content
    let mut resp = reqwest::get("https://github.com/FireWall-e/rusTIG/raw/master/target/release/git-crepo.exe").expect("request failed");
    
    // Get git executables path
    let output = Command::new("git")
                         .arg("--exec-path")
                         .output()
                         .expect("Error while executing 'git --exec-path' command!");

    println!("output is {:?}", output);
    // Raw path
    let mut git_executives_path =  str::from_utf8(&output.stdout).unwrap().to_owned();
    // Remove new line symbol
    git_executives_path.truncate(git_executives_path.len() - 1);
    // Full path to executable
    git_executives_path = format!("{}\\git-crepo.exe", git_executives_path);
    println!("git_exe_path is  {}", git_executives_path);

    let mut out = File::create(git_executives_path).expect("Failed to create file");
    io::copy(&mut resp, &mut out).expect("Failed to copy content");
}