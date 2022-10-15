use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    println!("Logging SOME_VAR if present");

    for (key, value) in env::vars() {
        if key == "SOME_VAR" {
            println!("{}: {}", key, value);
        }
    }

    println!("notice SOME_VAR being present in .env but not being logged on Windows unless specified on system level");
}
