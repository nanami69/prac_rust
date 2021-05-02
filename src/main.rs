extern crate minigrep_nanami_20210502;

use std::env;
use std::process;

use minigrep_nanami_20210502::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 引数解析時に問題
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_nanami_20210502::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}