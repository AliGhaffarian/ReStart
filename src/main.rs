
#[path = "ui/main menu.rs"] mod main_menu;
use main_menu::UI;


#[path = "modules/appdb.rs"] mod appdb;

#[path = "modules/appclass.rs"] mod appclass;


#[path = "ui/utilities.rs"] mod utilities;

#[path = "modules/groups.rs"] mod groups;

use std::io::{self, stdin, Write};
use std::process::Command;

use std::fs::File;
use crate::appclass::LaunchInfo;


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

        let mut ui;
    match UI::load_from_json("files and groups.json") {
        Ok(some_ui) => {
            ui = some_ui;
        }
        Err(_) => {
            File::create("files and groups.json").expect("Failed to create file");
            ui = UI::new();
        }
    }

    loop {
        ui.main_menu();

        clear_console();
    }


    }