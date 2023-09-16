use std::ops::Index;
use std::string::ToString;
use serde::Deserialize;

pub mod appclass;
use appclass::App;

#[derive(Deserialize, Clone)]
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
    pub fn search(self, app : App)->i32
    {
        if (self.clone().exists(app.clone()) || self.clone().exists_name(app.clone().get_name())) == false {return -1;}

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
        1
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


    pub fn len(&self)->i32
    {
        self.apps.len() as i32
    }
    pub fn get_app(self, index : i32)->Option<App>
    {
        if self.clone().is_index_inbound(index) {return None;}

        Some(self.clone().apps[index as usize].clone())
    }


    pub fn is_index_inbound(self, index : i32)->bool
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

}