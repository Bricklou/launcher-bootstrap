mod autocomplete;
mod cli;

fn main() {
    match cli::run() {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {}
    }
}
