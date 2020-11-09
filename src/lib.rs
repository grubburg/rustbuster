use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use clap::{Arg, App, ArgMatches};
use std::process;
use std::fs;
use std::error::Error;
use std::sync::mpsc;


type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}


struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) 
        -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            
            match message {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }           
        });

        Worker{
            thread: Some(thread),
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
       ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            self.sender
                .send(Message::NewJob(job))
                .unwrap();            
        }

}

impl Drop for ThreadPool {
    fn drop (&mut self) {

        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .unwrap();
        }

        for worker in &mut self.workers {
            
            if let Some(thread)  = worker.thread.take() {

                thread.join().unwrap();
        
            }
        }
    }
}


pub struct Config {
    pub url:         String,
    pub wordlist:    String,
    pub threads:     usize 
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
            Some(arg) => arg.parse::<usize>().unwrap(),
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

    let pool = ThreadPool::new(config.threads); 

    for line in wordlist.lines() {
   
        let path = format!("{}/{}", config.url, line);
        
        pool.execute( move || {
             
            let res = reqwest::blocking::get(&path).unwrap();

            if res.status() == reqwest::StatusCode::OK {
                println!("{}", path);
            }    
        });
    }
    Ok(())
}
