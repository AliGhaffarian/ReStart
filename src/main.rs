#[path = "modules\\appclass.rs"] mod appclass;
#[path = "modules\\appclass.rs"] mod appdb;
use std::process::Command;
use appclass::App;
use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;



//reserved keywords : all
static RESERVED_KEYWORDS: Mutex<Vec<&str>> = Mutex::new(Vec::new());

fn main() {

    // Specify the path to the Windows application executable.
    let app_path = r#"C:\Program Files (x86)\Steam\steam.exe"#;

    let steam = App::new(app_path.to_string());



}
