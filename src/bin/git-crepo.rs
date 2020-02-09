use std::process::Command;

pub fn main() {
    // Open windows calc
    Command::new("calc").output();
}