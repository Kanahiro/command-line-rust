fn main() {
    let res = wcr::get_args();
    match res {
        Ok(cfg) => {
            wcr::run(cfg);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
