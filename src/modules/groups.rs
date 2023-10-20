use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord)]
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
