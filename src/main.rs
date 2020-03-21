use git2::Repository;
use gotham::handler::Handler;
use gotham::handler::HandlerFuture;
use gotham::handler::NewHandler;
use gotham::state::State;
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

fn say_hello(state: State) -> (State, &'static str) {
    (state, "Hello World!")
}

#[derive(Copy, Clone)]
struct MyCustomHandler;

impl NewHandler for MyCustomHandler {
    type Instance = Self;

    fn new_handler(&self) -> gotham::error::Result<Self> {
        Ok(*self)
    }
}

impl Handler for MyCustomHandler {
    fn handle(self, _state: State) -> Box<HandlerFuture> {}
}

fn do_web() {
    println!("do web");
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
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

#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;
    use http::StatusCode;

    #[test]
    fn receive_hello_world_response() {
        let test_server = TestServer::new(|| Ok(say_hello)).unwrap();
        let response = test_server
            .client()
            .get("http://localhost")
            .perform()
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.read_body().unwrap();
        assert_eq!(&body[..], b"Hello World!");
    }
}
