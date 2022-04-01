fn main() {
    if let Err(e) = washme::get_args().and_then(washme::run) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
