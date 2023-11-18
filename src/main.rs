
/*#[path = "ui/main menu.rs"] mod main_menu;
use main_menu::UI;
*/

//#[path = "modules/appdb.rs"] mod appdb;
#[path = "modules/applist.rs"] mod applist;

#[path = "modules/appclass.rs"] mod appclass;
#[path = "modules/utilities.rs"] mod back_utils;
#[path = "modules/custom_order.rs"] mod custom_order;

#[path = "modules/groups.rs"] mod groups;

use std::io::{self, stdin, Write};

use std::process::Command;


use std::fs::File;
use std::rc::Rc;
use::std::string;
use crate::appclass::{App, LaunchInfo};
use crate::applist::{AppList, AppListPrintData, DetailApproach};
use crate::back_utils::utils::insert_sorted;
use crate::custom_order::CustomOrder;

#[path = "ui/utilities.rs"] mod utilities;
use utilities::Util;
use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use crate::appclass::LaunchInfo::CustomCommand;
//use crate::applist::{AppList, AppListPrintData, DetailApproach};

fn main() {
/*        let mut ui;

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

*/     /*let printmethod = AppListPrintData{
                detail_options: DetailApproach::ProcessName { print_approach : LineApproach::MultiLine},
                prefix : Some("someprefix : ".to_string()),

                repeat_prefix_for_each_line : false,
                custom_ordered : false,
        };
*/


        let spotify = App::new(LaunchInfo::CantLaunch,"spotifyprocess".to_string(), None);
        let  steam = App::new(LaunchInfo::CustomCommand {command : "steamLaunchCommand".to_string(), args : vec!["steamarg1".to_string(), "steamarg2".to_string()]},"steanprocess".to_string(), Some("steamAlias".to_string()));
        let  ok = App::new(LaunchInfo::Address {address : "okAddress".to_string()},"okprocess".to_string(), Some("okAlias".to_string()));


    let mut appvec = Vec::<App>::new();


    insert_sorted(&mut appvec, ok, true);
    insert_sorted(&mut appvec, spotify, true);
    insert_sorted(&mut appvec, steam, true);






    let mut applist = AppList::new();
    let mut print_data = AppListPrintData{detail_options : DetailApproach::FullDetail, prefix_options : None, custom_ordered : true};

    let uvec = Vec::<usize>::from(vec![1,5,7,4,2,3,6,0]);

    println!("{}", CustomOrder::is_valid_order(&uvec));


}


