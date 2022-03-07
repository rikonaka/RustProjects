use std::env;
use std::fs;
use std::fmt;
use std::error::Error;
use std::sync::mpsc::channel;
use std::time::Duration;
use notify::{RecommendedWatcher, Watcher, RecursiveMode};
// use rusqlite::{params, Connection, Result};
use rusqlite::{params, Connection};
use notify;

#[derive(Debug)]
struct DiskChange {
    id: i64,
    log: Option<String>,
}

#[derive(Debug)]
struct OpenLogFileError;

impl Error for OpenLogFileError{}

impl fmt::Display for OpenLogFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Can not open the system log file!")
    }
}


fn watch(watch_path: &String) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(watch_path, RecursiveMode::Recursive)?;

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn open_log_cur(log_file: &Stirng, cursor: i32) -> Result<String, OpenFileError> {
    let contents = match fs::read_to_string(log_file) {
        Ok(o) => o,
        Err(e) => return Err(OpenFileError),
    }

    /*
    let indices: Vec<_> = match contents.match_indices("\n").collect() {
        Ok(o) => o,
        Err(e) => return Err(OpenFileError),
    }
    for i in indices {
        if (i[0] == cursor) {
        }
    }
    */
    let mut spl
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("no arg input");
    }
    let watch_path = &args[1];
    // println!("{:?}", args);
    match watch(watch_path) {
        Err(e) => println!("{:?}", e),
        _ => ()
    }
}
