use std::process;


fn main() {

    let config = rustbuster::config_from_args().unwrap();

    if let Err(e) = rustbuster::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

}
