#[path = "modules\\appclass.rs"] mod appclass;
use crate::appclass::{LaunchInfo, Names};
use crate::appclass::App;


//#[path = "modules\\appdb.rs"] mod appdb;
//#[path = "ui\\main menu.rs"] mod mainmenu;
/*use mainmenu::UI;
use mainmenu::appdb::AppDB;
use mainmenu::appdb::appclass::App;*/
use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;
use std::io::{self, stdin, Write};
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

//reserved keywords : all
static RESERVED_KEYWORDS: Mutex<Vec<&str>> = Mutex::new(Vec::new());

fn clear_console() {
    if cfg!(target_os = "windows") {
        // On Windows, use the "cls" command to clear the console.
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        // On Unix-like systems (including Linux and macOS), use ANSI escape codes to clear the console.
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        io::stdout().flush().unwrap();
    }
}



    fn main() {

        let mut spotify_protocol = LaunchInfo::Name { name : String::from("spotify")};

        let mut spotify_groups = Vec::<String>::new();

        spotify_groups= vec![String::from("music"), String::from("music"), String::from("app"), String::from("music"), String::from("some")];

        let mut spotify = App::new(spotify_protocol, "spotify.exe".to_string(), None);

        spotify.add_groups(spotify_groups);

        for index in spotify.search_groups(vec!["muSic".to_string(), "somE".to_string()] ){
            println!("{}", index);
        }

    }