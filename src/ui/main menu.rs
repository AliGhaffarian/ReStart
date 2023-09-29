
use std::io;
//get key after failing to kill or run and preferences
#[path = "../modules/appdb.rs"] pub mod appdb;




use serde::{Serialize, Deserialize};
use serde_json;

use std::fs::File;
use std::io::{Read, stdin};
use std::io::Write;

use crate::groups::{Group, Groups};
use crate::appclass::{App, LaunchInfo};
use crate::appdb::AppDB;
use crate::utilities::Util;

use std::string::ToString;

use std::string::String;


#[derive(Serialize, Deserialize, Clone, Default)]
struct UserPref{
    pub groups_included : bool,
    pub enable_alias : bool
}

impl UserPref {
    pub fn new()->Self{
        Self{
            groups_included : true,
            enable_alias : false
        }
    }
    pub fn print(&self){
        println!("1_print groups of each app : {}\n2_print alias instead of process name if available : {}", self.groups_included, self.enable_alias)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UI
{
    pub app_db : AppDB,
    pub saved : bool,
    user_pref : UserPref
}

impl UI {

    pub fn new()->Self
    {
        Self{
            app_db : AppDB::new(),
            saved : false,
            user_pref : UserPref::new()
        }

    }
    pub fn print_app_index(& self, index : usize)
    {
        let mut print_string : String;

        let app = match self.app_db.get_app(index){
            Some(an_app) => an_app,
            None => return,
        };


        if self.user_pref.enable_alias{
            print_string = match app.get_alias() {
                Some(alias)=>alias,
                None => app.get_process_name(),
            };
        }

        else {print_string = app.get_process_name();}

        print_string = format!("{} ", print_string);

        if self.user_pref.groups_included{
            for group in self.app_db.get_member_group_names_index(index)
            {
                print_string = format!("{}  {}", print_string, group)
            }
        }

        print!("{}", print_string)

    }
    pub fn print_all_apps(&self)
    {
        for i in 0..=self.app_db.apps_len() - 1
        {
            print!("{} _ ", i);
            self.print_app_index(i);
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
            "apps"=>self.print_all_apps(),
            "groups"=>self.print_all_groups(),
            _ =>return,
        }
        return
    }

    pub fn print_all_groups(&self){
        for group in &self.app_db.get_groups_all(){
            println!("name : {}  working directory : {}", group.get_name(), group.get_working_directory().unwrap_or("None".to_string()));
        }
        Util::get_key();
    }

    pub fn main_menu(&mut self)
    {
        if self.app_db.apps_len() == 0{
            println!("no apps detected enter help to get started!")
        }
        else{self.print_all_apps();}

        let mut input = String::new();

        match stdin().read_line(&mut input){
            Ok(_) =>{},
            _ =>{return},
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

            let _ :usize = match input.trim().parse() {
                Ok(index) => {
                    self.app_db.app_index_action(index, "restart", None);
                    0
                },
                _ =>{
                    self.app_db.app_groups_action(&vec![input.clone()], "restart");
                    0
                },
            };

        }
    }

    pub fn special_command_handler(&mut self, input_vec : Vec<String>)->bool
    {
        match input_vec[0].to_lowercase().trim()
        {
            "help" => Self::help(),
            "quit" => self.quit(),
            "reg" => self.regs(input_vec),
            "save" => self.save_to_json("files and groups.json").expect("file not found please make a \"files and groups\".json"),
            "del" => self.dels(input_vec),
            "group" => self.group_app(),
            "print" => self.prints(input_vec),
            "edit" => self.edits(input_vec),
            "swap" => self.swap(input_vec),
            _ =>return false,
        }
        true
    }

    pub fn change_user_prefrences(&mut self, mut input_vec : Vec<String>){

        if !input_vec.is_empty(){
            input_vec.remove(0);
        }
        let is_precommanded = !input_vec.is_empty();

        let mut method_input = String::new();

        if is_precommanded == false{
            self.user_pref.print();
            println!("enter number of any preference to toggle");
            stdin().read_line(&mut method_input);
        }
        else{
            method_input = input_vec[0].clone();
        }


        method_input = method_input.trim().to_string();


        let num = match method_input.parse(){
            Ok(num) => num,
            Err(err) =>{
                println!("{}", err);
                return
            }
        };

        match num{
            1 => {
                self.user_pref.groups_included = !self.user_pref.groups_included;
                return
                }
            2 => {
                self.user_pref.enable_alias = !self.user_pref.enable_alias;
                return
            }
            _ => return,
        }
    }

    pub fn edits(&mut self, mut input_vec : Vec<String>)
    {
        if !input_vec.is_empty(){
            input_vec.remove(0);}

        if input_vec.is_empty(){return}

        match input_vec[0].to_lowercase().trim(){
            "app" => self.edit_app_alias(input_vec),
            "group" => self.edit_group(input_vec),
            "pref" => self.change_user_prefrences(input_vec),
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
        if !input_vec.is_empty(){
            input_vec.remove(0);}

        if input_vec.is_empty(){return}

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
        println!("provide one of these launch methods:\n1_app's address(not tested on linux)\n2_app's name(not guaranteed to work)\n3_custom command");

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

                launch_info_input.clear();

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
        input = input.to_lowercase().trim().to_string();
        return match input.len(){
            0=>None,
            _=>Some(input)
        }
    }
    pub fn input_process_name()->Option<String>{
        println!("enter process name of your app (it will be used to kill the app )");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("failed get process name");

        input = input.to_lowercase().trim().to_string();

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
            stdin().read_line(&mut method_input).expect("failed to get input");
        }

        else{
            method_input = input_vec[0].clone();
        }

        let group_name = method_input.trim().to_string();
        method_input.clear();


        if input_vec.is_empty() == false {
            input_vec.remove(0);
        }
        let is_precommanded = input_vec.is_empty() == false;

        if is_precommanded == false{
            println!("[optional] enter working directory for this group of apps");
            stdin().read_line(&mut method_input).expect("failed to get input");
        }
        else{
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();
        let group_working_directory : Option<String>;

        match method_input.len() {
            0 => group_working_directory = None,
            _ => group_working_directory = Some(method_input)
        }

        if self.app_db.exists_group(&group_name) {
            println!("group already exists");
            Util::get_key();
            return;
        }

        let mut group = Group::new(group_name);
        group.set_working_directory(group_working_directory);

        self.saved = false;
        self.app_db.add_group(&group);
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

        match self.app_db.exists_app_process_name(&process_name){
            true =>{
                println!("app already exists");
                Util::get_key();
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

        let index = match self.app_db.search_process_name(&method_input){
            Some(index) => index,
            None => {
                println!("app not found");
                Util::get_key();
                return
            }
        };

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

        let group_input: Vec<String>;

        println!("enter process name of apps you want to group separated by |");

        stdin().read_line(&mut method_input).expect("failed to get list of app names");

        app_input = method_input.split('|').map(|s| s.trim().to_string()).collect();


        if self.app_list_validator(app_input.clone()) == false
        {
            println!("one or more app names were invalid");
            Util::get_key();
            return
        }

        method_input.clear();


        println!("enter name of groups separated by space");

        stdin().read_line(&mut method_input).expect("failed to get list of app names");

        group_input = method_input.split(' ').map(|s| s.trim().to_string()).collect();

        if self.group_list_validator(&group_input) == false
        {
            println!("one or more groups names were invalid");
            Util::get_key();
            return
        }

        self.saved = false;
        self.app_db.add_members_to_groups(&app_input, &group_input);

    }
    pub fn group_list_validator(&self, group_names : &Vec<String>)->bool
    {
        for group_name in group_names {

            if self.app_db.exists_group(&group_name.trim().to_string()) == false{
                return false;
            }
        }
        true
    }
    pub fn app_list_validator(&self, app_names : Vec<String>)->bool
    {
        for app_name in app_names {
            if self.app_db.exists_app_process_name(&app_name.trim().to_string()) == false{
                return false;
            }
        }
        true
    }

    pub fn dels(&mut self , mut input_vec : Vec<String>)
    {
        if !input_vec.is_empty(){
        input_vec.remove(0);}

        if input_vec.is_empty(){return}

        match input_vec[0].to_lowercase().trim()
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


        if self.app_db.exists_app_process_name(&method_input) == false{
            println!("app doesnt exists");

            println!("Press Enter to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
        }
        self.saved = false;
        self.app_db.remove_app_process_name(&method_input);
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
            stdin().read_line(&mut method_input).expect("failed to get input");
        }

        else{
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();

        match self.app_db.search_group_by_name(&method_input){
            Some(_)=>{
                self.app_db.remove_group_by_group_name(&method_input);
                self.saved = false;
            }
            None=>{
                println!("group not found");
                Util::get_key();
            },
        }

    }

    pub fn swap(&mut self, mut input_vec : Vec<String>){

        if !input_vec.is_empty(){
            input_vec.remove(0);
        }

        let is_precommanded = input_vec.len() >= 2;

        let mut method_input = String::new();
        let mut method_input_vec: Vec<String>;

        if !is_precommanded{
            println!("enter id of the two apps you wanna swap separated by space");
            stdin().read_line(&mut method_input).expect("failed to get id of apps");
            method_input_vec = method_input.split(' ').map(|s| s.to_string()).collect();
        }
        else {
            method_input_vec = input_vec;
        }

        if method_input_vec.len() < 2 {return}

        let mut nums = Vec::<usize>::new();

        for index in 0..2{
            match method_input_vec[index].trim().parse() {
                Ok(parsed_string) => nums.push(parsed_string),
                _ => return
            }
        }

        if self.app_db.swap(nums[0], nums[1]) == false{
            println!("invalid input");
            Util::get_key();
        }

    }

    pub fn help() {
        let string = concat!(
            "enter number of the app or a group to restart them \n" ,
            "you can enter number of apps and their group separated by spaces to restart them in a sequence\n\n" ,
            "-------------\n\n" ,
            "commands:\n\n" ,
            "reg [app/group] \n" ,
            "registers app or group name \n" ,
            "name launch method works if the app can be launch by its name in terminal \n\n" ,
            "del [app/group] \n" ,
            "delete an app or group \n\n" ,
            "group [apps] \n" ,
            "add a group for a list of apps by their process names\n\n" ,
            "edit [apps/pref]\n" ,
            "edit process name of an app or user preference\n\n" ,
            "save \n" ,
            "quit \n");


        println!("{}", string);
        Util::get_key();
    }
}


