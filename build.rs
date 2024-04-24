use std::path::PathBuf;
use std::process::Command;

const COMET_BFT: &str = "https://github.com/cometbft/cometbft.git";
const DEPS_DIR: &str = "deps";

const BRANCH: &str = "v0.37.x";

fn main() {
    let path: PathBuf = [DEPS_DIR, "cometbft"].iter().collect();

    if path.exists() {
        return // nothing to do - already installed
    }

    // git clone the repo
    let res = Command::new("git")
        .args(["clone", COMET_BFT, path.to_str().unwrap()])
        .output()
        .expect("Failed to execute 'git clone' command");

    if !res.status.success() {
        panic!("failed to clone cometbft repo")
    }
    
    // make sure we're on the right branch
    let res = Command::new("git")
        .current_dir(&path)
        .args(["checkout", BRANCH])
        .output()
        .expect("Failed to execute 'git checkout' command");

    if !res.status.success() {
        panic!("failed to switch branch on cometbft repo")
    }

    // install the abci
    let res = Command::new("make")
        .current_dir(&path)
        .args(["install_abci"])
        .output()
        .expect("Failed to execute 'make' command");

    if !res.status.success() {
        panic!("failed to install the abci")
    }

    // copy the abci-cli to the project root directory, for convenience
    let gopath = std::env::var("GOPATH").expect("GOPATH was not found. Is Go properly installed?");
    let source: PathBuf = [&gopath, "bin", "abci-cli"].iter().collect();
    let dest: PathBuf = ["abci-cli"].iter().collect();
    std::fs::copy(source, dest).expect("failed to copy the abci-cli to project root");
}
