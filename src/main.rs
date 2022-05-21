fn main() {
    // ERROR HANDLING
    if let Err(e) = chart::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
