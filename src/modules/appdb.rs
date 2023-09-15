use std::string::ToString;
use serde::Deserialize;

use appclass::App;

#[derive(Deserialize, Clone)]
struct AppDB
{
    apps : Vec<App>,
}

impl AppDB
{
    pub fn exists(self, app : App)->bool
    {
        true
    }

    pub fn add_app(&mut self, app : App)->bool
    {
        true
    }

    pub fn add_apps(&mut self, app : App)->i32
    {
        1
    }

}

impl AppDB
{
    pub fn add_app(self, app : App)
    {

    }

    pub fn exists(self, app : App)
    {

    }

    pub fn exists_name(self, name : String)
    {

    }

    pub fn search(self, app : App)->i32
    {
        if (!self.exists(app) || self.exists(app.name))
            return -1;

        for one_app in self.apps
        {
            if(one_app.get_name())
                return 1;
        }
    }


}