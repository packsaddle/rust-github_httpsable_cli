extern crate git_httpsable;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let username = match env::var("GIT_HTTPSABLE_USERNAME") {
        Ok(result) => result,
        Err(_) => panic!("GIT_HTTPSABLE_USERNAME is required."),
    };
    let password = "x-oauth-basic".to_string();
    let mut child = git_httpsable::run(&args[1..], &username, &password).unwrap();
    let ecode = child.wait().unwrap();
    std::process::exit(ecode.code().unwrap());
}
