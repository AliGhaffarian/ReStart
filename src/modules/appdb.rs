use std::ops::Index;
use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;

pub mod appclass;
use appclass::App;

#[derive(Serialize, Deserialize, Clone)]
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
    pub fn add_app(&mut self, app : App)
    {
        if self.clone().exists_name(app.clone().get_name()) == false {self.apps.push(app);}
    }

    pub fn exists(self, app : App)->bool
    {
        let app_name = app.get_name();

        for one_app in self.apps
        {
            if one_app.clone().get_name().eq(&app_name)
            {return true;}
        }
        false
    }

    pub fn exists_name(self, name : String)->bool
    {
        for app in self.apps
        {
            if app.clone().get_name().eq(name.clone().as_str())
            {return true;}
        }
        false
    }

    //not done
    pub fn search(self, app : App)->usize
    {
        for i in 0..= self.clone().len()
        {
            match self.clone().get_app(i){
                None => panic!(),

                Some(an_app)=>{
                    if an_app.get_name().as_str()
                        .eq(app.clone().get_name().as_str())
                            {return i;}
                }
            }
        }
        usize::MAX
    }
    pub fn search_name(self, name : String)->i32
    {
        if self.clone().exists_name(name.clone()) == false{return -1;}

        for i in 0..=self.apps.len()
        {
            if name.clone().eq(self.apps[i as usize].clone().get_name().as_str()){ return i as i32;}
        }

        -1
    }


    pub fn len(&self)->usize
    {
        self.apps.len()
    }
    pub fn get_app(self, index : usize)->Option<App>
    {
        if self.clone().is_index_inbound(index) {return None;}

        Some(self.clone().apps[index].clone())
    }


    pub fn is_index_inbound(self, index : usize)->bool
    {
        (index < 0 || index >= self.clone().len())
    }
    pub fn app_name_action(&mut self, app_name: String, action: &str) {

        let index = self.clone().search_name(app_name) as usize;

        self.app_index_action(index, action);
    }
    pub fn app_index_action(&mut self, index : usize, action : &str)
    {
        self.apps[index].action(action);
    }

    pub fn app_group_action(&mut self, groups : Vec<String>, action : &str)
    {
        let lookup_result = self.groups_lookup(groups);

        for i in lookup_result
        {
            self.app_index_action(i, action);
        }
    }

    pub fn groups_lookup(&self, groups : Vec<String>)->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for group in &groups {
            let indexes = self.group_lookup(group.clone());

            // Iterate through the indexes and add them to result if they haven't been added already
            for &index in &indexes {
                if !result.contains(&index) {
                    result.push(index);
                }
            }
        }

        result
    }

    pub fn group_lookup(&self, group : String)->Vec<usize>
    {
        let mut result = Vec::<usize>::new();
        for i   in 0..=self.len() -1
        {
            if self.apps[i as usize].clone().get_groups().contains(&group) {
                result.push(i as usize);
            }
        }
        result
    }

    pub fn remove_app(&mut self, app : App)
    {
        let index = self.clone().search(app);

        if index == usize::MAX
        {
            return;
        }
        self.apps.remove(index as usize);
    }

    pub fn remove_app_groups(&mut self, groups : Vec<String>)
    {
        let indexes = self.groups_lookup(groups);

        for i in indexes
        {
            self.apps.remove(i);
        }
    }

    pub fn remove_groups(&mut self, groups : Vec<String>)
    {
        let indexes = self.groups_lookup(groups.clone());

        for i in indexes
        {
            self.apps[i as usize].rem_groups(groups.clone());
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
}