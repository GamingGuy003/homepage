use std::process::Command;

fn main() {
    // create GIT_HASH env variable for output in page
    let git_hash_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to get commit hash");

    let git_repo_output = Command::new("git")
        .args(["config", "--get", "remote.origin.url"])
        .output()
        .expect("Failed to get remote url");

    // create DATE env variable for output in page
    let date_output = Command::new("date").output().expect("Failed to get date");

    let git_hash = String::from_utf8(git_hash_output.stdout).expect("Failed to parse commit");
    let date = String::from_utf8(date_output.stdout).expect("Failed to parse date");
    let git_repo_raw = String::from_utf8(git_repo_output.stdout).expect("Failed to parse repo");

    let git_repo = git_repo_raw
        .split('/')
        .next_back()
        .unwrap_or_default()
        .trim_end_matches(".git");

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rustc-env=DATE={date}");
    // set the base path to repo name if compiling on github actions
    if std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("cargo::rustc-env=BASE_PATH=/{git_repo}");
    }
}
