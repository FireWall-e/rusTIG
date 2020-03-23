extern crate reqwest;

use std::str;
use std::env;
use std::process::Command;

// https://developer.github.com/v3/repos/#delete-a-repository
fn main() {
    println!("Processing...");

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();

    // Retrieve token from git config
    let mut output = Command::new("git")
                             .arg("config")
                             .arg("rustig.username")
                             .output()
                             .expect("GitHub username is not specified");

    // Assign username
    let github_username = str::from_utf8(&output.stdout).unwrap().to_owned().replace('\n', "");

    // Retrieve token from git config
    output = Command::new("git")
                     .arg("config")
                     .arg("rustig.token")
                     .output()
                     .expect("GitHub username is not specified");
    // Assign token
    let github_access_token = str::from_utf8(&output.stdout).unwrap().to_owned().replace('\n', "");

    // https://developer.github.com/v3/repos/#parameters-4
    let request_url = format!(
        "https://api.github.com/repos/{username}/{repository_name}",
        username = github_username,
        repository_name = args[1]
    );

    // Related topics:
    // https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html#create-and-delete-gist-with-github-api
    let client = reqwest::Client::new();
    let response = client.delete(&request_url)
                         .basic_auth(&github_username, Some(&github_access_token))
                         .send()
                         .unwrap();

    // Check response status
    // Expected 204
    if response.status().is_success() {
        println!("\nDeleted successfully! :)");
    } else {
        println!("Error occurred during repository deletion :(");
    }
}