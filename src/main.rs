use std::{env, process};
use minigrep::{Config, run};

// from: https://course.rs/basic-practice/intro.html
// cargo run -- the poem.txt
// cargo run -- the poem.txt true
// IGNORE_CASE=1 cargo run -- the poem.txt
// cargo test
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // eprintln! 可以进行重定向输出
        // cargo run > output.txt
        eprintln!("parse config error: {}", err);
        process::exit(1);
    });

    println!("search for {}", config.query);
    println!("file path {}", config.path);

    if let Err(e) = run(config) {
        eprintln!("run error: {}", e.to_string());
        process::exit(1);
    }
}
