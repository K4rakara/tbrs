#![feature(try_blocks)]

extern crate chrono;
extern crate clap;
extern crate failure;
extern crate serde;
extern crate serde_json;

use std::env;

use chrono::{ DateTime, Utc };
use clap::{ App, Arg };
use failure::Error;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Board {
    name: String,
    tasks: HashMap<usize, Box<dyn Item>>,
}

trait Item<'a>: Deserialize<'a> + Serialize + Debug + Clone {
    fn id(&self) -> usize;
    fn star(&mut self);
    fn starred(&self) -> bool;
    fn archive(&mut self);
    fn archived(&self) -> bool;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Task {
    id: usize,
    archived: bool,
    complete: bool,
    created: DateTime<Utc>,
    description: String,
    priority: Option<usize>,
    starred: bool,
}

impl<'a> Item<'a> for Task {
    fn id(&self) -> usize { self.id }
    fn star(&mut self) { self.starred = !self.starred; }
    fn starred(&self) -> bool { self.starred }
    fn archive(&mut self) { self.archived = !self.archived; }
    fn archived(&self) -> bool { self.archived }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Note {
    id: usize,
    archived: bool,
    created: DateTime<Utc>,
    starred: bool,
}

impl<'a> Item<'a> for Task {
    fn id(&self) -> usize { self.id }
    fn star(&mut self) { self.starred = !self.starred; }
    fn starred(&self) -> bool { self.starred }
    fn archive(&mut self) { self.archived = !self.archived; }
    fn archived(&self) -> bool { self.archived }
}

fn main() {
    let result: Result<(), Error> = try {
        let matches = App::new("taskbook-rs")
            .version("0.0.0")
            .author("Jack Johannesen <jack@insertdomain.name>")
            .about("A reimplementation of taskbook in rust.");
    };
    match result {
        Ok(()) => (),
        Err(error) => {
            println!("Encountered a runtime error: {}", error);
            exit(1);
        },
    }
}

