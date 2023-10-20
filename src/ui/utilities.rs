use std::io;
use std::process::Command;
use std::io::Write;
pub struct Util;

impl Util
{
    pub fn get_key() {
        println!("press Enter to continue");
        let mut input= "".to_string();
        let _ = io::stdin().read_line(&mut input).unwrap();
    }

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
}