

use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;


use crate::utilities::Util;
use crate::groups::{Group, Groups};
use crate::appclass::App;
use crate::appclass::LaunchInfo;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppDB
{
    groups : Groups,
    apps : Vec<App>,
}

impl AppDB
{
    pub fn new () -> Self
    {
        Self{
            apps : Vec::<App>::new(),
            groups : Groups::new()
        }
    }
    //true if app process name doesnt exist and register is successful
    pub fn add_app(&mut self, app : &App)->bool
    {
        if self.exists_app_process_name(&app.get_process_name()) == false {
            self.apps.push(app.clone());
            return true
        }
        false
    }

    //launch info is set to CantLaunch or not
    pub fn can_run_index(&mut self, index : usize)->bool
    {
        match self.is_index_inbound(index){
            true => self.apps[index].get_launch_info() != LaunchInfo::CantLaunch,
            false => false,
        }
    }

    //set launch info by index false if index is out of bound
    pub fn set_app_launch_info_index(&mut self, index : usize , launch_info : LaunchInfo)->bool
    {
        return match self.is_index_inbound(index) {
            true => self.apps[index].set_launch_info(launch_info),
            false => false,
        }
    }

    //set app alias by index false if index is out of bound
    pub fn set_app_alias_index(&mut self, index : usize , input : &String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps[index].set_alias(input.clone());
                true
            },
            false => false,
        }
    }

    //set app process name by index false if index is out of bound
    pub fn set_app_process_name_index(&mut self, index : usize , input : &String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps[index].set_process_name(input.clone());
                true
            },
            false => false,
        }
    }
    pub fn set_app_launch_info_process_name(&mut self, process_name : &String, launch_info : LaunchInfo)->bool{
        return match self.search_process_name(process_name){
            None => false,
            Some(index) => self.apps[index].set_launch_info(launch_info),
        }
    }

    // reset launch info to CantLaunch by process name
    pub fn reset_app_launch_info_process_name(&mut self, process_name : &String){

        return match self.search_process_name(process_name){

            None => return,

            Some(index) => {
                self.apps[index].reset_launch_info();
            },

        }

    }

    //get app launch info by process name
    pub fn get_app_launch_info_process_name(&mut self, process_name : &String)->Option<LaunchInfo>{
        return match self.search_process_name(process_name) {
            None => None,
            Some(index) => Some(self.apps[index].get_launch_info()),
        }
    }

    //uses process name
    pub fn exists_app(&self, app : &App) ->bool
    {
        self.exists_app_process_name(&app.get_process_name())
    }

    pub fn exists_app_process_name(&self, name : &String) ->bool
    {
        self.search_process_name(name) != None

    }

    //uses process name
    pub fn search(&self, app : &App)->Option<usize>
    {
        self.search_process_name(&app.get_process_name())
    }
    pub fn search_process_name(&self, name : &String) -> Option<usize>
    {
        let to_lower_name = name.trim().to_lowercase().to_string();

        if self.apps_len() == 0{return None}

        for i in  0..=self.apps_len() - 1{
            if self.apps[i].get_process_name().eq(&to_lower_name){
                return Some(i)
            }
        }
        
        None
    }

    pub fn search_process_names(& self, apps_process_name : &Vec<String>) ->Vec<usize>{

        let mut result = Vec::<usize>::new();

        for process_name in apps_process_name{
            match self.search_process_name(process_name){
                Some(index)=>result.push(index),
                None=>continue,
            }
        }

        result
    }

    pub fn apps_len(&self) ->usize
    {
        self.apps.len()
    }

    pub fn groups_len(&self) ->usize{self.groups.len()}

    //get app by index None if index is out of bound
    pub fn get_app(&self, index : usize)->Option<App>
    {
        return match self.is_index_inbound(index) {
            true => Some(self.apps[index].clone()),
            false =>None,
        }
    }


    pub fn is_index_inbound(&self, index : usize)->bool
    {
        index < self.apps_len()
    }

    //searches by process name and passes action
    pub fn app_name_action(&mut self, app_process_name: &String, action: &str)->bool {

        return match self.search_process_name(app_process_name){

            None => false,

            Some(index) =>{
                self.app_index_action(index, action, None);
                true
            },

        }
    }
    //core of running apps if group isn't None checks for working directory in there
    pub fn app_index_action(&mut self, index : usize, action : &str, group : Option<&Group>) ->bool
    {
        return match self.is_index_inbound(index){
            true => {
                match group{
                    Some(group) => {
                        self.apps[index].action(action, group.get_working_directory());
                        true
                    },
                    None => {
                        self.apps[index].action(action, None);
                        true
                    }
                }/*end of group match*/
            },
            false => false
        }/*match self.is_index_inbound*/
    }
    //app action by groups
    pub fn app_groups_action(&mut self, group_names : &Vec<String>, action : &str)->i32
    {

        let mut failed_attempts = 0;

        for group_name in group_names{
            self.app_group_action(group_name.clone(), action);
        }

        failed_attempts
    }
    //app action for every member of group if group name is valid returns failed attempts
    pub fn app_group_action(&mut self, group_name : String, action : &str)->i32{

        let group : Group;

        let mut failed_attempts = 0;

        group = match self.groups.search_group_name(&group_name) {
            Some(index) => match self.groups.get_group_by_index(&index){
                Some(a_group) => a_group,
                None => {
                    return -1;
                },
            }
            None => {
                return -1;
            }
        };

        for member_index in group.get_members(){
            if self.app_index_action(member_index, action, Some(&group)) == false {failed_attempts += 1}
        }

        failed_attempts
    }

    //takes a Vec of group names and returns all unique member indexes
    pub fn groups_members_by_group_name(&self, groups : &Vec<String>) ->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for group in groups {
            let indexes = self.group_members_by_group_name(group);

            // Iterate through the indexes and add them to result if they haven't been added already
            for &index in &indexes {
                if !result.contains(&index) {
                    result.push(index);
                }
            }
        }
        result
    }

    //gets a group and returns all member indexes
    pub fn group_members_by_group_name(&self, name: &String) ->Vec<usize>
    {
        return match self.groups.search_group_name(name){
            Some(index) => self.groups.get_group_by_index(&index).unwrap().get_members(),
            None => Vec::<usize>::new(),
        }
    }

    //uses remove app name
    pub fn remove_app(&mut self, app : &App)->bool
    {
        self.remove_app_process_name(&app.get_process_name())
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
    pub fn swap(&mut self, first_index : usize , second_index : usize)->bool{

        if (self.is_index_inbound(first_index) || self.is_index_inbound(second_index)) == false{
            return false
        }

        self.groups.swap_members_global(first_index, second_index);
        self.apps.swap(first_index, second_index);
        true
    }

    //adds an apps index in groups
    pub fn add_members_to_groups(&mut self, member_process_names: &Vec<String>, groups : &Vec<String>)
    {

        for process_name in member_process_names
        {
            match self.search_process_name(process_name){

                None =>{
                    println!("app {} not found", process_name);
                    Util::get_key();
                    return
                },

                Some(index) =>{
                    self.groups.add_member_to_groups(groups, index)
                },
            }
        }
    }




}