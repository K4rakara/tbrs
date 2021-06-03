extern crate serde;

use crate::item;

use std::collections::HashMap;

use serde::{ Serialize, Deserialize };

use item::Item;

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    items: HashMap<usize, Item>, 
}

