

use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;
use std::collections::HashMap;
use std::rc::Rc;
use crate::utilities::Util;
use crate::groups::Group;
use crate::appclass::App;
use crate::appclass::LaunchInfo;
use crate::applist::AppList;
use crate::back_utils::utils::is_index_inbound;

#[derive(Clone, Default)]
pub struct AppDB
{
    grouped_apps : HashMap<Group, AppList>,
    apps : AppList,
}

impl AppDB
{
    pub fn new () -> Self
    {
        Self{
            grouped_apps : HashMap::<Group, AppList>::new(),
            apps : AppList::new(),
        }
    }

    pub fn apps_binary_search_process_name_based(apps : &Vec<Rc<App>>, process_name : String)->Result<usize, usize>{
        todo!()
    }

    pub fn insert_app(&mut self, app : App, sorted : bool)->bool{
        todo!()
    }

    //set launch info by index false if index is out of bound
    pub fn set_app_launch_info_index(&mut self, index : usize , launch_info : LaunchInfo)->bool
    {
        todo!("edit all occurrences");
        return match is_index_inbound(&self.apps.apps, index) {
            true => self.apps.apps[index].set_launch_info(launch_info),
            false => false,
        }

    }

    //set app alias by index false if index is out of bound
    pub fn set_app_alias_index(&mut self, index : usize , input : &String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps.apps[index].set_alias(input.clone());
                true
            },
            false => false,
        }
    }

    //set app process name by index false if index is out of bound
    pub fn set_app_process_name_index(&mut self, index : usize , input : String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps.apps[index].set_process_name(input);
                true
            },
            false => false,
        }
    }
    pub fn search_process_name(&self, name : &String) -> Option<usize>
    {
        self.apps.apps_binary_search_process_name_based(name.to_owned())
    }

    //core of running apps if group isn't None checks for working directory in there
    pub fn app_index_action(&mut self, index : usize, action : &str, working_directory: Option<String>) ->bool
    {
        return match self.is_index_inbound(index){
            true => {
                self.apps[index].action(action, working_directory)
            },
            false => false
        }/*match self.is_index_inbound*/
    }


    //removes app index from all groups before removing
    pub fn remove_app_process_name(&mut self, app_process_name : &String) ->bool
    {
        return match self.search_process_name(app_process_name) {
            None => false,

            Some(index) => {
                self.groups.remove_member_global(index);
                self.apps.remove(index);
                true
            },
        }
    }

    //removes an apps index from a group
    pub fn remove_app_from_group(&mut self, process_name : String, group_name: &String) ->bool
    {
        let process_name_index = match self.search_process_name(&process_name){
            Some(index)=>index,
            None=>return false
        };
        self.groups.remove_member(process_name_index, group_name.clone())
    }

    //returns failed attempts
    pub fn remove_groups_by_group_name(&mut self, group_names: &Vec<String>)->i32
    {
        self.groups.remove_groups(group_names.clone())
    }
    pub fn remove_group_by_group_name(&mut self, group_name : &String)->bool{
        self.groups.remove_group(group_name.clone())
    }

    pub fn save_to_json(filename: &str, data: &AppDB) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string(data)?;

        let mut file = File::create(filename)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    pub fn load_from_json(filename: &str) -> Result<AppDB, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let app_db: AppDB = serde_json::from_str(&json_data)?;

        Ok(app_db)
    }

    //swaps the two app index from all groups and in apps
    pub fn swap(&mut self, first_index : usize , second_index : usize, group_name : Option<String>)->bool{

        todo!("make custom order");
        if (self.is_index_inbound(first_index) || self.is_index_inbound(second_index)) == false{
            return false
        }

        self.apps.swap(first_index, second_index);
        true
    }

    //adds an apps index in groups
    pub fn add_member_to_group(&mut self, app_index : usize, group_name : String, is_custom_ordered : bool) {
        todo!();
        let app_real_index = self.apps.to_ordered_index(app_index, is_custom_ordered);
        let group = Group{name : group_name};
        self.grouped_apps[group]

    }

    pub fn group_names_of_app(& self, index : usize) ->Vec<String>{
        todo!()
    }
    pub fn get_group_names_all(& self)->Vec<String>{
        todo!()
    }
    pub fn get_groups_all(& self)->Vec<Group>{
        todo!()
    }
    pub fn exists_group(& self, group_name : &String)->bool{
        todo!()
    }

    pub fn add_group(&mut self, group : &Group)->bool{
        todo!()
    }


}