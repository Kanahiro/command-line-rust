fn main() {
    if let Err(e) = cutr::get_args().and_then(cutr::run) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
