#[path = "modules\\appclass.rs"] mod appclass;
use appclass::App;

use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;



//reserved keywords : all
static RESERVED_KEYWORDS: Mutex<Vec<&str>> = Mutex::new(Vec::new());

/*
#[derive(Deserialize, Clone)]
struct App
{
    name : String,
    address : String,
    groups : Vec<String>,
}

impl App
{
    fn new(address : String)->Self
    {
        Self
        {
            address : address.clone(),
            name : App::get_name(address.clone()),
            groups : Vec::<String>::new(),

        }
    }

    // extracts name of the app by its address
    fn get_name(address : String)->String
    {
        address.rsplit("\\").next ().unwrap ().to_string()
    }
    // sets name property using get_name
    fn set_name(&mut self)
    {
        self.name = App::get_name(self.address.clone());
    }

    fn exists_group(self, group : String) -> bool
    {
        for group_in_self in  self.groups
        {
            if group.eq(&group_in_self){
                return true;
            }
        }
        false
    }

    //returns amount of failed pushes already existing causes failing to push
    fn add_groups(&mut self, groups : Vec<String>) -> i32
    {
        let mut failed_pushes = 0;
        for group in groups
        {
            if self.to_owned().exists_group(group.clone()) {
                failed_pushes += 1 }

            else {
                self.groups.push(group )}
        }
        return failed_pushes;
    }
    fn add_group(&mut self, group : String) -> bool
    {
        if self.to_owned().exists_group(group.clone()){ return false;}

        self.groups.push(group.clone());

        true
    }
    //returns amount of failed removes
    fn rem_groups(&mut self, groups : Vec<String>)->i32 {

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
    fn rem_group(&mut self, group : String)->bool {

        if self.clone().exists_group(group.clone()) {

            let index = self.groups.iter().position(|n| n.eq(&group));
            self.groups.remove(index.unwrap());
            return true;
        }
        false
    }

    fn set_address(&mut self, address : String)
    {
        self.address = address.clone();
        self.name = App::get_name(address);
    }
}



struct Apps
{
    apps : Vec<App>,
}
*/




fn main() {
    let app = appclass::App::new("dqwdwq".to_string());


    RESERVED_KEYWORDS.lock().unwrap().push("all");
    println!("{}", RESERVED_KEYWORDS.lock().unwrap().first().unwrap());
}
