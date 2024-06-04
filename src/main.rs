mod libs;
use libs::cli::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}