use std::string::ToString;
use serde::Deserialize;





#[derive(Deserialize, Clone)]
pub struct App
{
    name: String,
    address: String,
    groups: Vec<String>,
}

impl App
{
    pub fn new(address: String) -> Self
    {
        Self
        {
            address: address.clone(),
            name: App::get_name(address.clone()),
            groups: Vec::<String>::new(),

        }
    }

    // extracts name of the app by its address
    pub fn get_name(address: String) -> String
    {
        address.rsplit("\\").next().unwrap().to_string()
    }
    // sets name property using get_name
    pub fn set_name(&mut self)
    {
        self.name = App::get_name(self.address.clone());
    }

    pub fn exists_group(self, group: String) -> bool
    {
        for group_in_self in self.groups
        {
            if group.eq(&group_in_self) {
                return true;
            }
        }
        false
    }

    //returns amount of failed pushes already existing causes failing to push
    pub fn add_groups(&mut self, groups: Vec<String>) -> i32
    {
        let mut failed_pushes = 0;
        for group in groups
        {
            if self.to_owned().exists_group(group.clone()) {
                failed_pushes += 1
            } else {
                self.groups.push(group)
            }
        }
        return failed_pushes;
    }
    pub fn add_group(&mut self, group: String) -> bool
    {
        if self.to_owned().exists_group(group.clone()) { return false; }

        self.groups.push(group.clone());

        true
    }
    //returns amount of failed removes
    pub fn rem_groups(&mut self, groups: Vec<String>) -> i32 {
        let mut failed_removes = 0;

        for group in groups {
            if self.clone().exists_group(group.clone()) {
                let index = self.groups.iter().position(|n| n.eq(&group));
                self.groups.remove(index.unwrap());
                failed_removes += 1;
            }
        }
        failed_removes
    }
    //true if deletes
    pub fn rem_group(&mut self, group: String) -> bool {
        if self.clone().exists_group(group.clone()) {
            let index = self.groups.iter().position(|n| n.eq(&group));
            self.groups.remove(index.unwrap());
            return true;
        }
        false
    }

    pub fn rem_groups_all(&mut self)
    {
        self.groups.clear();
    }
    pub fn set_address(&mut self, address: String)
    {
        self.address = address.clone();
        self.name = App::get_name(address);
    }
}



