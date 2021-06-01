extern crate chrono;
extern crate serde;

use std::any::TypeId;
use std::fmt::{ self, Debug };

use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };

pub trait Item<'a>: Serialize + Deserialize<'a> + Debug + Clone {
    fn id(&self) -> usize;
    
    fn archived(&self) -> bool;
    
    fn archive(&mut self);
    
    fn starred(&self) -> bool;
    
    fn star(&mut self);
    
    fn type(&self) -> TypeId;

    fn downcast<'s, T>(&'s self) -> Option<&'s T>
    where
        T: Item<'a>,
    
    fn downcast_mut<'s, T>(&'s self) -> Option<&'s mut T>
    where
        T: Item<'a>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Completion {
    Completed,
    Working,
    Pending,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    id: usize,
    archived: bool,
    completion: Completion,
    description: String,
    priority: usize,
    starred: bool,
}

impl<'a> Item<'a> for Task {
    fn id(&self) -> usize { self.id }
    
    fn archived(&self) -> bool { self.archived }

    fn archive(&mut self) { self.archived = !self.archived; }
    
    fn starred(&self) -> bool { self.starred }

    fn star(&mut self) { self.starred = !self.starred; }
    
    fn type(&self) -> TypeId { TypeId::of::<Task>() }
    
    fn downcast<'s, T>(&self) -> Option<&'s T> {
        if TypeId::of::<T>() == TypeId::of::<Task>() {
            Some(&self)
        } else {
            None
        }
    }
    
    fn downcast_mut<'s, T>(&'s mut self) -> Option<&'s mut T> {
        if TypeId::of::<T>() == TypeId::of::<Task>() {
            Some(&mut self)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    id: usize,
    archived: bool,
    description: String,
    starred: bool,
}

impl<'a> Item<'a> for Note {
    fn id(&self) -> usize { self.id }
    
    fn archived(&self) -> bool { self.archived }

    fn archive(&mut self) { self.archived = !self.archived; }
    
    fn starred(&self) -> bool { self.starred }

    fn star(&mut self) { self.starred = !self.starred; }
    
    fn type(&self) -> TypeId { TypeId::of::<Note>() }
    
    fn downcast<'s, T>(&self) -> Option<&'s T> {
        if TypeId::of::<T>() == TypeId::of::<Note>() {
            Some(&self)
        } else {
            None
        }
    }
    
    fn downcast_mut<'s, T>(&'s mut self) -> Option<&'s mut T> {
        if TypeId::of::<T>() == TypeId::of::<Note>() {
            Some(&mut self)
        } else {
            None
        }
    }
}

