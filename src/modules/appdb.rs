use std::ops::Index;
use std::string::ToString;
use serde::Deserialize;

mod appclass;
use appclass::App;

#[derive(Deserialize, Clone)]
pub struct AppDB
{
    apps : Vec<App>,
}


impl AppDB
{
    pub fn add_app(self, app : App)
    {

    }

    pub fn exists(self, app : App)->bool
    {
        true
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

        1
    }

    pub fn restart_group(self, groups : Vec<String>)
    {
    }
    pub fn len(self)->i32
    {
        self.apps.len() as i32
    }
    pub fn get_app(self, index : i32)->Option<App>
    {
        if self.clone().is_index_inbound(index) {return None;}

        Some(self.clone().apps[index as usize].clone())
    }
    pub fn kill_group(self, group : Vec<String>)
    {

    }

    pub fn is_index_inbound(self, index : i32)->bool
    {
        (index < 0 || index >= self.clone().len())
    }
    pub fn app_action(&mut self, app_name: String, action: &str) {

        if let .[Some(index) = self.search_name(app_name) {
            self.apps[index].action(action);
        } else {
            // Handle the case where the app is not found.
            // You can choose to return an error or take some other action here.
        }
    }

    pub fn kill_index(self, index : i32)->bool
    {
        if self.is_index_inbound(index)
        {
            //self.clone().kill();
            return true;
        }
        false
    }

    pub fn restart_index(self, index : i32)->bool
    {
        true
    }


}