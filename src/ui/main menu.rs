use std::io;
//get key after failing to kill or run and preferences
#[path = "..\\modules\\appdb.rs"] pub mod appdb;
mod utilities;
use utilities::util;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::{Read, stdin};
use std::io::Write;
use appdb::AppDB;
use std::string::ToString;
use serde_json::Value::String as otherString;
use crate::appclass::App;
use std::string::String;
use serde::de::Unexpected::Str;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UI
{
    pub app_db : AppDB,
    pub saved : bool,
    pub defined_groups : Vec<String>,
}

impl UI {

    pub fn new()->Self
    {
        Self{
            app_db : AppDB::new(),
            saved : false,
            defined_groups : Vec::<String>::new(),
        }
    }
    pub fn print_app_index(& self, index : usize, groups_included : bool)
    {
        let mut print_string : String;

        let app = self.app_db.clone().get_app(index).unwrap();

        print_string = app.clone().get_name();

        if groups_included{
            for group in app.get_groups()
            {
                print_string = format!("{} {}", print_string, group)
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
        for group in &self.defined_groups{
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

        stdin().read_line(&mut input).expect("Failed to read line");

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

            if self.defined_groups.contains(&input)
            {
                self.app_db.app_group_action(vec![input.clone()], "restart")
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
            _ =>return false,
        }
        true
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
            "app" => self.reg_app(input_vec),
            "group" =>self.reg_group(&mut input_vec),
            _ => println!("invalid command"),
        }
    }

    pub fn reg_app(&mut self, mut input_vec : Vec<String>)
    {

        let mut method_input= String::new();
        if input_vec.is_empty() == false {
            input_vec.remove(0);
        }

        let is_pre_commanded = input_vec.is_empty() == false;

        if is_pre_commanded == false {
            println!("enter address of the app");
            stdin().read_line(&mut method_input).expect("failed to get input");
        }

        if(is_pre_commanded) {
            method_input = input_vec[0].clone();
        }

        method_input = method_input.trim().to_string();

        let app = appdb::appclass::App::new((method_input));


        if self.clone().app_db.exists(app.clone()){
            println!("app exists");

            println!("Press Enter to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
    }
        self.saved = false;
        self.app_db.add_app(app);
    }
    pub fn reg_group(&mut self, mut input_vec : &mut Vec<String>)
    {
        let mut method_input = "".to_string();

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

        if self.defined_groups.contains(&method_input.trim().to_string()) {
            println!("group already exists");
            util::get_key();
            return;
        }
        self.saved = false;
        self.defined_groups.push(method_input);

    }

    pub fn edit_group(&mut self, input_vec : Vec<String>)
    {

    }

    pub fn edit_app_name(&mut self, mut input_vec : Vec<String>)
    {
        if input_vec.is_empty() == false{
            input_vec.remove(0);
        }
        let mut is_precommanded = input_vec.is_empty();

        let mut method_input = String::new();

        if is_precommanded == false {
            println!("enter name of the app on the list");
            stdin().read_line(&mut method_input).expect("failed to get app name");
        }
        else { method_input = input_vec[0].clone(); }

        method_input = method_input.trim().to_string();

        let index = self.app_db.clone().search_name(method_input.clone()) as usize;

        if index == usize::MAX{
            println!("app not found");
            util::get_key();
            return;
        }

        if input_vec.is_empty() == false{
            input_vec.remove(0);
        }
        is_precommanded = input_vec.is_empty();


        if is_precommanded == false {
            println!("enter the new app name");
            stdin().read_line(&mut method_input).expect("failed to get new app name");
        }
        else{
            method_input = input_vec[0].clone();
        }

        self.saved = false;
        self.app_db.set_app_name_index(index , method_input);

    }

    pub fn group_app(&mut self)
    {
        let mut app_input = Vec::<String>::new();
        let mut method_input = String::new();
        let mut group_input = Vec::<String>::new();
        println!("enter name of apps you want to group seperated by |");

        io::stdin().read_line(&mut method_input).expect("failed to get list of app names");

        app_input = method_input.split('|').map(|s| s.to_string()).collect();

        if self.app_list_validator(app_input.clone()) == false
        {
            println!("one or more app names were invalid");
            util::get_key();
            return
        }

        method_input.clear();


        println!("enter name of groups separated by space");

        io::stdin().read_line(&mut method_input).expect("failed to get list of app names");

        group_input = method_input.split(' ').map(|s| s.to_string()).collect();

        if self.group_list_validator(group_input.clone()) == false
        {
            println!("one or more groups names were invalid");
            util::get_key();
            return
        }
        self.saved = false;
        self.app_db.add_group(app_input, group_input);

    }
    pub fn group_list_validator(&self, group_names : Vec<String>)->bool
    {
        for group_name in group_names {

            if self.defined_groups.contains(&group_name.trim().to_string()) == false{
                return false;
            }
        }
        true
    }
    pub fn app_list_validator(&self, app_names : Vec<String>)->bool
    {
        for app_name in app_names {
            if self.app_db.clone().exists_name(app_name.trim().to_string()) == false{
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


        if self.clone().app_db.exists_name(method_input.clone()) == false{
            println!("app doesnt exists");

            println!("Press Enter to continue...");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
        }
        self.saved = false;
        self.app_db.remove_app_name(method_input);
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

        if self.defined_groups.contains(&method_input.trim().to_string()) == false {
            println!("group doesnt exists");
            util::get_key();
            return;
        }
        self.saved = false;

        let index = self.defined_groups.iter().position(|x| x.eq(&method_input)).expect("group not found");

        self.app_db.remove_groups(vec![method_input]);
        self.defined_groups.remove(index);
    }
    pub fn help() {
        println!("enter number of the app or a group to restart them \n
        you can enter number of apps and their group separated by spaces to restart them in a sequence\n
        reg [app/group] \n
        del [app/group] \n
        group [apps] \n
        save \n
        quit \n");
        util::get_key();
    }
}


