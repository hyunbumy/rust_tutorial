use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // For things like `collect`, we need to annotate the type so that it can infer.
    let args: Vec<String> = env::args().collect();

    // Like a closure for handling Result
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Use `eprintln` to write to STDERR
        eprintln!("Problem parsing arguemtns: {err}");
        process::exit(1);
    });

    // Use if let since we only care about handling the error and not the success result.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

