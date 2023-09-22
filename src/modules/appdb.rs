use std::ops::Index;
use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;

#[path = "..\\ui\\utilities.rs"] mod utilities;
use utilities::util;
pub mod appclass;
use appclass::App;
use appclass::LaunchInfo;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppDB
{
    apps : Vec<App>,
}


impl AppDB
{
    pub fn new () -> Self
    {
        Self{
            apps : Vec::<App>::new(),
        }
    }
    pub fn add_app(&mut self, app : &App)
    {
        if self.exists_name(&app.get_process_name()) == false {self.apps.push(app.clone());}
    }

    pub fn can_run_index(&mut self, index : usize)->bool
    {
        match self.is_index_inbound(index){

            true => self.apps[index].get_launch_info() != LaunchInfo::CantLaunch,
            false => false,

        }
    }
    pub fn set_app_launch_info_index(&mut self, index : usize , launch_info : LaunchInfo)->bool
    {
        return match self.is_index_inbound(index) {
            true => self.apps[index].set_launch_info(launch_info),
            false => false,
        }
    }
    pub fn set_app_alias_index(&mut self, index : usize , input : &String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps[index].set_alias(input.clone());
                true
            },
            false => false,
        }
    }

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
            usize::MAX => false,
            index => self.apps[index].set_launch_info(launch_info),
        }
    }
    pub fn reset_app_launch_info_process_name(&mut self, process_name : &String){

        return match self.search_process_name(process_name){

            usize::MAX => return,

            index => {
                self.apps[index].reset_launch_info();
            },

        }

    }
    pub fn get_app_launch_info_process_name(&mut self, process_name : &String)->Option<LaunchInfo>{
        return match self.search_process_name(process_name) {
            usize::MAX => None,
            index => Some(self.apps[index].get_launch_info()),
        }
    }

    pub fn exists(&self, app : &App)->bool
    {
        self.exists_name(&app.get_process_name())
    }

    pub fn exists_name(&self, name : &String)->bool
    {
        self.search_process_name(name) != usize::MAX
    }

    //not done
    pub fn search(&self, app : &App)->usize
    {
        self.search_process_name(&app.get_process_name())
    }
    pub fn search_process_name(&self, name : &String) -> usize
    {
        let to_lower_name = name.trim().to_lowercase().to_string();

        for i in  0..=self.len() - 1{
            if self.apps[i].get_process_name().eq(&to_lower_name){
                return i
            }
        }
        usize::MAX
    }


    pub fn len(&self)->usize
    {
        self.apps.len()
    }
    pub fn get_app(&self, index : usize)->Option<App>
    {
        return match self.is_index_inbound(index) {
            true => Some(self.apps[index].clone()),
            false =>None,
        }
    }


    pub fn is_index_inbound(&self, index : usize)->bool
    {
        index < self.len()
    }
    pub fn app_name_action(&mut self, app_process_name: &String, action: &str)->bool {

        return match self.search_process_name(app_process_name){

            usize::MAX => false,

            index =>{
                self.app_index_action(index, action);
                true
            },

        }
    }
    pub fn app_index_action(&mut self, index : usize, action : &str)->bool
    {
        return match self.is_index_inbound(index){
            true => {
                self.apps[index].action(action);
                true
            },
            false => false
        }
    }

    pub fn app_group_action(&mut self, groups : &Vec<String>, action : &str)
    {
        let lookup_result = self.groups_lookup(groups);

        for i in lookup_result
        {
            self.app_index_action(i, action);
        }
    }

    pub fn groups_lookup(&self, groups : &Vec<String>)->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for group in groups {
            let indexes = self.group_lookup(group);

            // Iterate through the indexes and add them to result if they haven't been added already
            for &index in &indexes {
                if !result.contains(&index) {
                    result.push(index);
                }
            }
        }
        result
    }

    pub fn group_lookup(&self, group : &String)->Vec<usize>
    {
        let mut result = Vec::<usize>::new();
        for i   in 0..=self.len() -1
        {
            if self.apps[i].exists_group(&group){result.push(i)};
        }
        result
    }

    pub fn remove_app(&mut self, app : &App)->bool
    {
        return match self.search(&app) {
            usize::MAX => false,

            index => {
                self.apps.remove(index);
                true
            },
        }
    }

    pub fn remove_app_name(&mut self, app_process_name : &String)->bool
    {
        return match self.search_process_name(app_process_name) {
            usize::MAX => false,

            index => {
                self.apps.remove(index);
                true
            },
        }
    }

    pub fn remove_app_groups(&mut self, groups : &Vec<String>)
    {
        let indexes = self.groups_lookup(groups);

        for i in indexes
        {
            self.apps.remove(i);
        }
    }

    pub fn remove_groups(&mut self, groups : &Vec<String>)
    {
        let indexes = self.groups_lookup(groups);

        for i in indexes
        {
            self.apps[i].rem_groups(groups.clone());
        }
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

    pub fn add_group(&mut self, apps : &Vec<String>, groups : &Vec<String>)
    {
        for app in apps
        {
            match self.search_process_name(app){

                usize::MAX =>{
                    println!("app {} not found", app);
                    util::get_key();
                    return
                },

                index =>{
                    self.apps[index].add_groups(groups.clone());
                    return
                },

            }
        }
    }
}