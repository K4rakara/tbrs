extern crate chrono;
extern crate serde;

use std::fmt::{ self, Debug };

use chrono::{ DateTime, Utc };
use serde::{ Serialize, Deserialize };
use serde::ser::Serializer;
use serde::de::{ self, Deserializer, Unexpected, Visitor };

#[derive(Debug, Clone)]
pub enum Completion {
    Completed,
    Working,
    Pending,
}

impl Serialize for Completion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Completion::Completed => "completed",
            Completion::Working => "working",
            Completion::Pending => "pending",
        })
    }
}

impl<'de> Deserialize<'de> for Completion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CompletionVisitor;
        
        impl<'de> Visitor<'de> for CompletionVisitor {
            type Value = Completion;
            
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string \"completed\", \"working\", or \"pending\"")
            }
            
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match s {
                    "completed" => Ok(Completion::Completed),
                    "working" => Ok(Completion::Working),
                    "pending" => Ok(Completion::Pending),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
                }
            }
        }
        
        deserializer.deserialize_str(CompletionVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Item {
    Task {
        id: usize,
        description: String,
        created: DateTime<Utc>,
        completion: Completion,
        priority: usize,
        archived: bool,
        starred: bool,
    },
    Note {
        id: usize,
        description: String,
        created: DateTime<Utc>,
        archived: bool,
        starred: bool,
    },
}

impl Item {
    #[inline]
    pub fn id(&self) -> usize {
        match self {
            Item::Task { id, .. } => *id,
            Item::Note { id, .. } => *id,
        }
    }
    
    #[inline]
    pub fn identify(&mut self, i: usize) {
        match self {
            Item::Task { ref mut id, .. } => { *id = i; },
            Item::Note { ref mut id, .. } => { *id = i; },
        }
    }
    
    #[inline]
    pub fn description<'s>(&'s self) -> &'s str {
        match self {
            Item::Task { description, .. } => description,
            Item::Note { description, .. } => description,
        }
    }
    
    #[inline]
    pub fn describe<S>(&mut self, d: S)
    where
        S: AsRef<str>,
    {
        match self {
            Item::Task { ref mut description, .. } => {
                *description = d.as_ref().to_owned();
            },
            Item::Note { ref mut description, .. } => {
                *description = d.as_ref().to_owned();
            },
        }
    }
    
    #[inline]
    pub fn archived(&self) -> bool {
        match self {
            Item::Task { archived, .. } => *archived,
            Item::Note { archived, .. } => *archived,
        }
    }
    
    #[inline]
    pub fn archive(&mut self) {
        match self {
            Item::Task { ref mut archived, .. } => { *archived = !*archived; },
            Item::Note { ref mut archived, .. } => { *archived = !*archived; },
        }
    }
    
    #[inline]
    pub fn starred(&self) -> bool {
        match self {
            Item::Task { starred, .. } => *starred,
            Item::Note { starred, .. } => *starred,
        }
    }
    
    #[inline]
    pub fn star(&mut self) {
        match self {
            Item::Task { ref mut starred, .. } => { *starred = !*starred; },
            Item::Note { ref mut starred, .. } => { *starred = !*starred; },
        }
    }
}

