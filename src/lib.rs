
use clap::{Arg, App, ArgMatches};
use std::process;
use std::fs;
use std::error::Error;
use std::io::Read;



pub struct Config {
    pub url:         String,
    pub wordlist:    String,
    pub threads: u32
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
        let threads = match args.value_of("threads") {
            Some(arg) => arg.parse::<u32>().unwrap(),
            None => 4
        };
        Ok( Config { url, wordlist, threads } )

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
        .arg(Arg::with_name("threads")
             .short("t")
             .long("threads")
             .takes_value(true)
             .help("Number of threads (default 4)"))
        .get_matches();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Argument Error: {}", err);
        process::exit(1);
    });
    Ok(config)    
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
   
    let wordlist = fs::read_to_string(config.wordlist)?;

    for line in wordlist.lines() {
   
        let path = format!("{}/{}", config.url, line);

        let mut res = reqwest::blocking::get(&path)?;
        println!("{}", res.status());


    }
    println!("{}", config.url);
    Ok(())
}
