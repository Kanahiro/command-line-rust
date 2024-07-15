fn main() {
    let cfg = catr::get_args().unwrap();

    if let Err(e) = catr::run(cfg) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
