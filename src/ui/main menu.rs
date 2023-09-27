
use std::io;
//get key after failing to kill or run and preferences
#[path = "../modules/appdb.rs"] pub mod appdb;




use serde::{Serialize, Deserialize};
use serde_json;

use std::fs::File;
use std::io::{Read, stdin};
use std::io::Write;

use crate::appclass::{App, LaunchInfo, Names};
use crate::appdb::AppDB;
use crate::utilities::util;

use std::string::ToString;

use std::string::String;

use serde::de::Unexpected::Str;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UI
{
    pub app_db : AppDB,
    pub saved : bool,
    pub defined_groups : Names,
}

impl UI {
    
    pub fn new()->Self
    {
        Self{
            app_db : AppDB::new(),
            saved : false,
            defined_groups : Names::new(),
        }
    }
    pub fn print_app_index(& self, index : usize, groups_included : bool)
    {
        let mut print_string : String;

        let app = match self.app_db.get_app(index){
            Some(an_app) => an_app,
            None => return,
        };


        print_string = match app.get_alias() {
            Some(alias)=>alias,
            None => app.get_process_name(),
        };

        print_string = format!("{} ", print_string);

        if groups_included{
            for group in app.get_groups().get_all()
            {
                print_string = format!("{}  {}", print_string, group)
            }
        }

        print!("{}", print_string)

    }
    pub fn print_all_apps(&self, groups_included : bool)
    {
        for i in 0..=self.app_db.len() - 1
        {
            print!("{} _ ", i);
            self.print_app_index(i, groups_included);
            println!();
        }
    }

    pub fn prints(&self, mut input_vec : Vec<String>)
    {
        if input_vec.is_empty(){
            return;
        }
        input_vec.remove(0);

        match input_vec[0].trim() {
            "apps"=>self.print_all_apps(true),
            "groups"=>self.print_all_groups(),
            _ =>return,
        }
        return
    }

    pub fn print_all_groups(&self){
        for group in &self.defined_groups.get_all(){
            println!("{}", group);
        }
        util::get_key();
    }

    pub fn main_menu(&mut self)
    {
        if self.app_db.len() == 0{
            println!("no apps detected enter help to get started!")
        }
        else{self.print_all_apps(true);}

        let mut input = String::new();

        match stdin().read_line(&mut input){
            Ok(_) =>{},
            error =>{return},
        }

        if input.trim().len() == 0 {return}

        let input_vec: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

        self.input_handler(input_vec);

    }


    pub fn input_handler(&mut self, input_vec : Vec<String>)
    {

        if self.special_command_handler(input_vec.clone()) {
            return
        }


        for mut input in input_vec
        {
            input = input.trim().to_string();

            if util::is_all_digits(&input) {
                self.app_db.app_index_action(input.trim().parse().unwrap(), "restart");
                continue;
            }

            if self.defined_groups.exists(&input)
            {
                self.app_db.app_group_action(&vec![input.clone()], "restart")
            }


        }
    }

    pub fn special_command_handler(&mut self, input_vec : Vec<String>)->bool
    {
        match input_vec[0].trim()
        {
            "help" => Self::help(),
            "quit" => self.quit(),
            "reg" => self.regs(input_vec),
            "save" => self.save_to_json("files and groups.json").expect("file not found please make a \"files and groups\".json"),
            "del" => self.dels(input_vec),
            "group" => self.group_app(),
            "print" => self.prints(input_vec),
            "edit" => self.edits(input_vec),
            _ =>return false,
        }
        true
    }

    pub fn edits(&mut self, mut input_vec : Vec<String>)
    {
        if input_vec.is_empty(){return}
        input_vec.remove(0);
        match input_vec[0].trim(){
            "app" => self.edit_app_alias(input_vec),
            "group" => self.edit_group(input_vec),
            _ => return,
        }
    }
    pub fn quit(&mut self)
    {
        if self.saved == false
        {
            let mut input = String::new();
            println!("you made changes that haven't been saved wanna save them now? Y/N");
            stdin().read_line(&mut input).expect("failed to get input");
            if input.trim() == "Y"{
                self.save_to_json("files and groups.json").expect("failed to save");
                self.saved = true;
            }
        }

        std::process::exit(0);
    }
    pub fn save_to_json(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string(self)?;

        let mut file = File::create(filename)?;
        file.write_all(json_data.as_bytes())?;

        self.saved = true;

        Ok(())
    }

    pub fn load_from_json(filename: &str) -> Result<UI, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let ui: UI = serde_json::from_str(&json_data)?;

        Ok(ui)
    }

    pub fn regs(&mut self, mut input_vec : Vec<String>)
    {
        if input_vec.is_empty(){return};
        input_vec.remove(0);

        match input_vec[0].trim()
        {
            "app" => {
                self.reg_app(input_vec);

            },

            "group" =>{
                self.reg_group(input_vec);
            },

            _ => println!("invalid command"),
        }
    }


    pub fn input_launch_info()->Option<LaunchInfo>{
        println!("provide one of these launch methods:\n1_app's address\n2_app's name(not guaranteed to work\n3_custom command");
    
        let mut num_input = String::new();
        let mut launch_info_input = String::new();

        stdin().read_line(&mut num_input).expect("failed get num_input");

        return match num_input.trim(){
            "1"=>{
                println!("enter apps address");
                stdin().read_line(&mut launch_info_input).expect("failed get address");
                Some(LaunchInfo::Address{address : launch_info_input.trim().to_string()})
                },
            "2"=>{
                println!("enter apps name");
                stdin().read_line(&mut launch_info_input).expect("failed get app name");
                Some(LaunchInfo::Name{name : launch_info_input.trim().to_string()})
            }
            "3"=>{

                println!("enter the command");
                stdin().read_line(&mut launch_info_input).expect("failed get command");
                let input_command = launch_info_input.clone();

                println!("enter args");
                stdin().read_line(&mut launch_info_input).expect("failed get args");
                let input_args : Vec<String>= launch_info_input.split(' ').map(|s| s.trim().to_string()).collect();
                Some(LaunchInfo::CustomCommand{command : input_command.trim().to_string(),args : input_args})
            }
            _=>None
        }
            
    }

    pub fn input_alias()->Option<String>{

        println!("[optional]enter an alias for your app (it will be used to display in the app list)");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("failed get alias");
        input = input.trim().to_string();
        return match input.len(){
            0=>None,
            _=>Some(input)
        }
    }
    pub fn input_process_name()->Option<String>{
        println!("enter process name of your app (it will be used to kill the app )");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed get process name");

        input = input.trim().to_string();

        return match input.len(){
            0=>None,
            _=>Some(input)
        }
    }

    pub fn reg_group(&mut self, mut input_vec: Vec<String>){
        let mut method_input = String::new();

        if input_vec.is_empty() == false {
            input_vec.remove(0);
        }
        let is_precommanded = input_vec.is_empty() == false;

        if is_precommanded == false{
            println!("enter name of group");
            io::stdin().read_line(&mut method_input).expect("failed to get input");
        }

        else{
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();

        if self.defined_groups.exists(&method_input) {
            println!("group already exists");
            util::get_key();
            return;
        }
        self.saved = false;
        self.defined_groups.add(method_input);
    }

    pub fn reg_app(&mut self, mut input_vec : Vec<String>)->bool
    {
        let mut launch_info = LaunchInfo::CantLaunch;

        match Self::input_launch_info(){
            Some(a_launch_info) => {
                launch_info.set(a_launch_info);
            },
            None => return false
        }

        let process_name : String;
        match Self::input_process_name(){
            Some(an_input_process_name) => process_name = an_input_process_name,
            None => return false
        }

        match self.app_db.exists_name(&process_name){
            true =>{
                println!("app aleady exists");
                util::get_key();
                return false;
            }
            false =>{}
        }

        let alias : Option<String>;

        match Self::input_alias(){
            Some(an_alias) => alias = Some(an_alias),
            None => alias = None
        }
        
        

        let app = App::new(launch_info, process_name, alias);
        self.app_db.add_app(&app);

        true
        

    }

    pub fn edit_group(&mut self, input_vec : Vec<String>)
    {

    }

    pub fn print_all_process_names(& self){
        
    }

    pub fn edit_app_alias(&mut self, mut input_vec : Vec<String>)
    {
        if input_vec.is_empty() == false{
            input_vec.remove(0);
        }

        let mut is_precommanded = input_vec.is_empty() == false;

        let mut method_input = String::new();

        if is_precommanded == false {
            println!("enter process name of the app");
            stdin().read_line(&mut method_input).expect("failed to get app name");
        }
        else { method_input = input_vec[0].clone(); }

        method_input = method_input.trim().to_string();

        let index = self.app_db.clone().search_process_name(&method_input) as usize;

        if index == usize::MAX{
            println!("app not found");
            util::get_key();
            return;
        }

        if input_vec.is_empty() == false{
            input_vec.remove(0);
        }
        is_precommanded = input_vec.is_empty() == false;

        method_input.clear();

        if is_precommanded == false {
            println!("enter the new app name");
            stdin().read_line(&mut method_input).expect("failed to get new app name");
        }
        else{
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();

        self.saved = false;
        self.app_db.set_app_process_name_index(index , &method_input);

    }

    pub fn group_app(&mut self)
    {
        let app_input : Vec<String>;
        let mut method_input = String::new();
        let group_input;
        println!("enter process name of apps you want to group seperated by |");

        io::stdin().read_line(&mut method_input).expect("failed to get list of app names");

        app_input = method_input.split('|').map(|s| s.trim().to_string()).collect();


        if self.app_list_validator(app_input.clone()) == false
        {
            println!("one or more app names were invalid");
            util::get_key();
            return
        }

        method_input.clear();


        println!("enter name of groups separated by space");

        io::stdin().read_line(&mut method_input).expect("failed to get list of app names");

        group_input = method_input.split(' ').map(|s| s.trim().to_string()).collect();

        if self.group_list_validator(&group_input) == false
        {
            println!("one or more groups names were invalid");
            util::get_key();
            return
        }
        self.saved = false;
        self.app_db.add_group(&app_input, &group_input);

    }
    pub fn group_list_validator(&self, group_names : &Vec<String>)->bool
    {
        for group_name in group_names {

            if self.defined_groups.exists(&group_name.trim().to_string()) == false{
                return false;
            }
        }
        true
    }
    pub fn app_list_validator(&self, app_names : Vec<String>)->bool
    {
        for app_name in app_names {
            if self.app_db.exists_name(&app_name.trim().to_string()) == false{
                return false;
            }
        }
        true
    }

    pub fn dels(&mut self , mut input_vec : Vec<String>)
    {
        if input_vec.is_empty(){return;}
        input_vec.remove(0);

        match input_vec[0].trim()
        {
            "app" => self.del_app(input_vec),
            "group" => self.del_group(input_vec),
            _ => return,
        }
    }

    pub fn del_app(&mut self, mut input_vec : Vec<String>)
    {
        let mut method_input= String::new();
        if input_vec.is_empty() == false {
            input_vec.remove(0);
        }

        let is_pre_commanded = input_vec.is_empty() == false;

        if is_pre_commanded == false {
            println!("enter name of the app");
        }

        if is_pre_commanded {
            method_input = input_vec[0].clone();
        }

        else { io::stdin().read_line(&mut method_input).expect("failed to get input"); }

        method_input = method_input.trim().to_string();


        if self.app_db.exists_name(&method_input) == false{
            println!("app doesnt exists");

            println!("Press Enter to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
        }
        self.saved = false;
        self.app_db.remove_app_name(&method_input);
    }

    pub fn del_group(&mut self, mut input_vec : Vec<String>)
    {
        let mut method_input = "".to_string();

        if input_vec.is_empty() == false {
            input_vec.remove(0);
        }
        let is_precommanded = input_vec.is_empty() == false;

        if is_precommanded == false{
            println!("enter name of group you want to delete");
            io::stdin().read_line(&mut method_input).expect("failed to get input");
        }

        else{
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();

        if !self.defined_groups.exists(&method_input) == false {
            println!("group doesnt exists");
            util::get_key();
            return;
        }
        self.saved = false;

        self.app_db.remove_groups(&vec![method_input.clone()]);
        self.defined_groups.rem(method_input);
    }
    pub fn help() {
        println!("enter number of the app or a group to restart them \nyou can enter number of apps and their group separated by spaces to restart them in a sequence\n\n-------------\ncommands:\nreg [app/group] \ndel [app/group] \ngroup [apps] \nedit [apps]\nsave \nquit \n");
        util::get_key();
    }
}


