use git2::Repository;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().expect("Could not read head");
    let head_name = head.name().expect("Could not get head name");
    println!("Current head: {}", head_name);
}
