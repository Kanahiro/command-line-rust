fn main() {
    if let Err(e) = tailr::get_args().and_then(tailr::run) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
