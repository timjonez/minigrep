use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new();
    println!("Args: {:?}", config);
    println!("Searching for \"{}\" in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
