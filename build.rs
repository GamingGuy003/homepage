use std::process::Command;

fn main() {
    // create GIT_HASH env variable for output in page
    let git_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    // create DATE env variable for output in page
    let date_output = Command::new("date").output().unwrap();

    let git_hash = String::from_utf8(git_output.stdout).unwrap();
    let date = String::from_utf8(date_output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rustc-env=DATE={date}");
}
