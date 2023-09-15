#[path = "modules\\appclass.rs"] mod appclass;

use std::process::Command;
use appclass::App;
use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;



//reserved keywords : all
static RESERVED_KEYWORDS: Mutex<Vec<&str>> = Mutex::new(Vec::new());

#[derive(Deserialize, Clone)]
struct Apps
{
    apps : Vec<App>,
}

impl Apps
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

fn main() {

    // Specify the path to the Windows application executable.
    let app_path = r#"C:\Program Files (x86)\Steam\steam.exe"#;

    let steam = App::new(app_path.to_string());


    steam.clone().restart();
}
