
fn main() {

    let config = rustbuster::config_from_args().unwrap();
    println!("{}", config.url);
}
