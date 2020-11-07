use clap::{Arg, App, ArgMatches};
use std::process;

pub struct Config {
    pub url:         String,
    pub wordlist:    String,
    //pub num_threads: u8
}


impl Config {
    pub fn new( args: ArgMatches) -> Result<Config, &'static str> {
        
        let url = match args.value_of("url") {
            Some(arg) => String::from(arg),
            None => return Err("missing URL")
        };
        let wordlist = match args.value_of("wordlist") {
            Some(arg) => String::from(arg),
            None => return Err("missing wordlist")
        };
        Ok( Config { url, wordlist } )

    }
}

pub fn config_from_args() -> Result<Config, &'static str> {
    let args = App::new("rustbuster")
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .takes_value(true)
             .help("URI to rustbust"))
        .arg(Arg::with_name("wordlist")
             .short("w")
             .long("wordlist")
             .takes_value(true)
             .help("Wordlist to feed"))
        .get_matches();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Argument Error: {}", err);
        process::exit(1);
    });
    Ok(config)    
}

