#[path = "modules\\appclass.rs"] mod appclass;
use appclass::App;

use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;



//reserved keywords : all
static RESERVED_KEYWORDS: Mutex<Vec<&str>> = Mutex::new(Vec::new());


fn main() {
    let app = appclass::App::new("dqwdwq".to_string());


    RESERVED_KEYWORDS.lock().unwrap().push("all");
    println!("{}", RESERVED_KEYWORDS.lock().unwrap().first().unwrap());
}
