use git2::Repository;
use std::env;

fn get_current_head() -> String {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    let head = repo.head().expect("Could not read head");
    let head_name = head.name().expect("Could not get head name");
    head_name.into()
}

fn do_web() {
    println!("do web")
}

fn do_console() {
    println!("Current head: {}", get_current_head());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Specify 'web' or 'console'");
    }
    let first_arg = &args[1];
    match first_arg.as_str() {
        "web" => do_web(),
        "console" => do_console(),
        _ => panic!("Unknown command: {}", first_arg),
    }
}
