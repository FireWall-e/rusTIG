extern crate reqwest;

use std::str;
use std::env;
use std::process::Command;

// https://developer.github.com/v3/repos/#create-a-repository-for-the-authenticated-user
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

    let request_url = "https://api.github.com/user/repos";

    // Related topics:
    // https://stackoverflow.com/questions/28385884/how-to-create-repository-in-github-through-github-api
    // https://stackoverflow.com/questions/7870680/github-v3-api-create-a-repo
    let client = reqwest::Client::new();
    let request_body;

    // Request body parameters
    // If private flag has been provided
    if args.len() > 2 && args[2] == "--private" { 
        request_body = format!(
            "{{\"name\": \"{repository_name}\", \"private\": \"{private}\"}}",
            repository_name = args[1],
            private = true
        );
    } else { // Only repository name provided
        request_body = format!(
            "{{\"name\": \"{repository_name}\"}}",
            repository_name = args[1]
        );
    }
    
    // Send post request to GitHub api with parameters
    let response = client.post(request_url)
                         .basic_auth(&github_username, Some(&github_access_token))
                         .body(request_body)
                         .send()
                         .unwrap();
    
    // Check response status
    // Expected 201
    if response.status().is_success() {
        // Construct link to repository
        let created_repo_url = format!(
            "https://github.com/{username}/{repository_name}",
            username = github_username,
            repository_name = args[1]
        );
        println!("\nCreated successfully! :)");
        // Provide link
        println!("\n{}", created_repo_url);
    } else {
        println!("Error occurred during repository creation :(");
    }
}