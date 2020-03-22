extern crate reqwest;

use std::str;
use std::process::Command;
use std::io::{stdin, copy, prelude::Read};
use std::fs::File;

fn finish() {
    let mut stdin = stdin();
    println!("\nPress ENTER to exit...");
    // Read a single byte and discard
    stdin.read(&mut [0]).unwrap();
}

fn main() {
    // Git branch name
    let branch = "master";
    // Cargo build version
    let build_version = "release";
    // Created commands
    let git_commands = [
        "git-crepo", // Create GitHub repository
        "git-rustig" // Setup Git global config
    ].into_iter();
    // Get git executables path
    let output = Command::new("git")
                         .arg("--exec-path")
                         .output()
                         .expect("Error while executing 'git --exec-path' command!");
    // Raw path
    let mut git_executables_path =  str::from_utf8(&output.stdout).unwrap().to_owned();

    // Remove new line symbol
    git_executables_path.truncate(git_executables_path.len() - 1);
    println!("Git executables path is: {:?}", git_executables_path);

    // Iterate through created commands
    for command_name in git_commands {
        // Full path to executable
        git_executables_path = format!("{}/{}.exe", git_executables_path, command_name);
        println!("Path to the new executable is: {}", git_executables_path);

        // Build link to the githubs executable file
        let file_url = format!(
            "https://github.com/FireWall-e/rusTIG/raw/{}/target/{}/{}.exe",
            branch,
            build_version,
            command_name
        );

        // Get compiled executable content
        let mut file_content = reqwest::get(&file_url).expect("Request failed");
        // Create new empty file
        let mut file = File::create(&git_executables_path).expect("Failed to create file");
        // Put file_content inside file
        copy(&mut file_content, &mut file).expect("Failed to copy content");
    }

    // Terminate execution after key press
    finish();
}