use git2::Repository;
use std::env;

fn main() {
    let test_path = env::args().nth(1).expect("No path given");
    let repo = Repository::open(test_path).expect("Invalid repository path");

    for branch in repo.branches(None).unwrap() {
        println!("hihi");
    }
}
