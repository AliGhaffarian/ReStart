use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, PartialOrd, Eq)]
pub struct Group {
    pub name : String,
    pub working_directory : Option<String>,
}

impl Group {
    pub fn new(name : String)->Self {
        Self{
            name : name.trim().to_string(),
            working_directory : None,
        }
    }
}
impl Ord for Group{
    fn cmp(&self, other : &Self)->Ordering{
        self.name.cmp(&other.name)
    }
}