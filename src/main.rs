#[path = "modules\\appclass.rs"] mod appclass;
#[path = "modules\\appdb.rs"] mod appdb;
#[path = "ui\\main menu.rs"] mod mainmenu;
use mainmenu::UI;
use mainmenu::appdb::AppDB;
use mainmenu::appdb::appclass::App;
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
//group apps method
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