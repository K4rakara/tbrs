#![feature(try_blocks)]

extern crate chrono;
extern crate clap;
extern crate failure;
extern crate serde;
extern crate serde_json;

pub mod item;
pub mod board;

use std::collections::HashMap;
use std::env;
use std::fmt::{ self, Debug };

use chrono::{ DateTime, Utc };
use clap::{ App, Arg };
use failure::Error;
use serde::{ Deserialize, Serialize };

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
            println!("{}", error);
            std::process::exit(1);
        },
    }
}

