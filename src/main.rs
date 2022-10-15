use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        if key == "RUST_LOG" {
            println!("{}: {}", key, value);
        }
    }

    println!("logged all envs.");
}
