extern crate chrono;
extern crate serde;

use chrono::{ DateTime, Utc };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Task {
    archived: bool,
    complete: bool,
    created: DateTime<Utc>,
    description: String,
    priority: Option<usize>,
    starred: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Board {
    name: String,
    tasks: Vec<Task>,
}

fn main() {
}

