#[path = "modules\\appclass.rs"] mod appclass;
#[path = "modules\\appdb.rs"] mod appdb;
#[path = "ui\\main menu.rs"] mod mainmenu;
use mainmenu::UI;
use mainmenu::appdb::AppDB;
use mainmenu::appdb::appclass::App;
use std::string::ToString;
use std::sync::Mutex;
use serde::Deserialize;
use std::io::{self, Write};
use std::process::Command;


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



    // Specify the path to the Windows application executables.
    let steam_path = r#"C:\Program Files (x86)\Steam\steam.exe"#;
    let explorer_path = r#"C:\Windows\explorer.exe"#;
    let notepad_path = r#"C:\Windows\System32\notepad.exe"#;
    let chrome_path = r#"C:\Program Files\Google\Chrome\Application\chrome.exe"#;
    let firefox_path = r#"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe"#;
    let vlc_path = r#"C:\Program Files\VideoLAN\VLC\vlc.exe"#;
    let visual_studio_path = r#"C:\Program Files\Microsoft Visual Studio\VS2019\Common7\IDE\devenv.exe"#;
    let spotify_path = r#"C:\Program Files\Spotify\Spotify.exe"#;
    let discord_path = r#"C:\Users\YourUsername\AppData\Local\Discord\Update.exe"#;
    let winamp_path = r#"C:\Program Files (x86)\Winamp\winamp.exe"#;

    let mut app_db = AppDB::new();

    let mut steam = App::new(steam_path.to_string());
    let mut explorer = App::new(explorer_path.to_string());
    let mut notepad = App::new(notepad_path.to_string());
    let mut chrome = App::new(chrome_path.to_string());
    let mut firefox = App::new(firefox_path.to_string());
    let mut vlc = App::new(vlc_path.to_string());
    let mut visual_studio = App::new(visual_studio_path.to_string());
    let mut spotify = App::new(spotify_path.to_string());
    let mut discord = App::new(discord_path.to_string());
    let mut winamp = App::new(winamp_path.to_string());


    // Create groups for some apps
    steam.add_group("Games".to_string());
    explorer.add_group("System".to_string());
    notepad.add_group("Text_Editors".to_string());
    chrome.add_group("Browsers".to_string());
    firefox.add_group("Browsers".to_string());
    vlc.add_group("Media_Players".to_string());


    // Add the apps to the AppDB
    app_db.add_app(steam.clone());//0
    app_db.add_app(explorer.clone());//1
    app_db.add_app(notepad.clone());//2
    app_db.add_app(chrome.clone());//3
    app_db.add_app(firefox.clone());//4
    app_db.add_app(vlc.clone());//5
    app_db.add_app(visual_studio.clone());//6
    app_db.add_app(spotify.clone());//7
    app_db.add_app(discord.clone());//8
    app_db.add_app(winamp.clone());//9

    let mut epic_games = App::new(r#"C:\Program Files (x86)\Epic Games\Launcher\Portal\Binaries\Win32\EpicGamesLauncher.exe"#.to_string());
    epic_games.add_group("Games".to_string());
    app_db.add_app(epic_games);



    // Register apps with shared groups
    let apps_with_shared_groups = vec![steam.clone(), chrome.clone(), firefox, vlc, winamp];
    for mut app in apps_with_shared_groups {
        app_db.add_app(app.clone());
    }


    let mut ui = UI{app_db : app_db, commands : vec![], saved : false, defined_groups : vec![]};

    loop {
        ui.main_menu();
        clear_console();
    }
}