// highly inspired and partly copied from https://github.com/AmrDeveloper/GQL/blob/master/crates/gitql-core/src/object.rs

use std::any::Any;
use std::vec::Vec;
use std::prelude::rust_2015::String;
use std::vec;

#[derive(Debug)]
#[deprecated]
pub enum Value {
    Str(String),
    List(Vec<Value>),
    // Map(HashMap<String, Value>)
    // Object(Box<dyn Any>)
    Object(CartographyObject)
}

/// In memory representation of the list of [`Value`] in one Row
#[derive(Default, Debug)]
pub struct Row {
    pub values: Vec<Value>,
}

/// In memory representation of the Rows of one [`Group`]
#[derive(Default, Debug)]
pub struct Group {
    pub rows: Vec<Row>,
}

impl Group {
    /// Returns true of this group has no rows
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Returns the number of rows in this group
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// In memory representation of the GitQL Object which has titles and groups
#[derive(Default, Debug)]
#[deprecated]
pub struct CartographyObject {
    pub titles: Vec<String>,
    pub groups: Vec<Group>,
}

impl CartographyObject {
    /// Flat the list of current groups into one main group
    pub fn flat(&mut self) {
        let mut rows: Vec<Row> = vec![];
        for group in &mut self.groups {
            rows.append(&mut group.rows);
        }

        self.groups.clear();
        self.groups.push(Group { rows })
    }

    /// Returns true of there is no groups
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns the number of groups in this Object
    pub fn len(&self) -> usize {
        self.groups.len()
    }
}