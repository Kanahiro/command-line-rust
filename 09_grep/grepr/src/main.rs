fn main() {
    if let Err(e) = grepr::get_args().and_then(grepr::run) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
