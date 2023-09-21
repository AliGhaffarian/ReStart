use std::env::args;
use std::fs::read;
use std::io;
use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::process::{Child, Command, Stdio};
use serde_json;
use std::io::Read;
use std::io::Write;
#[path = "..\\ui\\utilities.rs"] pub mod utilities;
use utilities::util;
use crate::appclass::LaunchInfo::CantLaunch;

//need to get operating system
struct UserPrefrence
{
    restart_error: bool,
    restart_msg: bool,
    app_msg: bool,
}



#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Names
{
    names : Vec<String>,
}


impl Names {
    pub fn new()->Self
    {
        Self{
            names : Vec::<String>::new()
        }
    }
    pub fn exists(&self, name : &String) -> bool
    {
        for name_in_self in &self.names
        {
            if name.eq(name_in_self) {
                return true;
            }
        }
        false
    }

    //returns amount of failed pushes already existing causes failing to push
    pub fn adds(&mut self, names : Vec<String>) -> i32
    {
        let mut failed_pushes = 0;

        for mut name in names
        {
            name = name.trim().to_string();

            if self.exists(&name) {
                failed_pushes += 1
            }
            else {
                self.names.push(name)
            }
        }
        return failed_pushes;
    }
    pub fn add(&mut self, mut name: String) -> bool
    {
        name = name.trim().to_string();

        if self.exists(&name) { return false; }

        self.names.push(name);

        true
    }
    //returns amount of failed removes
    pub fn rems(&mut self, names: Vec<String>) -> i32 {

        let mut failed_removes = 0;

        for mut name in names {

            name = name.trim().to_string();

            if self.exists(&name) {

                let index = self.search(&name);

                self.names.remove(index);

                failed_removes += 1;
            }
        }
        failed_removes
    }
    //true if deletes
    pub fn rem(&mut self, mut name: String) -> bool {

        name = name.trim().to_string();

        if self.exists(&name) {

            let index = self.search(&name);

            self.names.remove(index);

            return true;
        }

        false
    }

    pub fn search(&self, name: &String) ->usize
    {
        match self.names.iter().position(|n| n.eq(name)){
            Some(index)=>return index,
            None => usize::MAX,
        }
    }

    pub fn clear(&mut self)
    {
        self.names.clear();
    }

    pub fn get_all(&self)->Vec<String>
    {
        self.names.clone()
    }

    pub fn get(&self, index : &usize)->String
    {
        if *index < self.names.len()
        {
            return self.names[*index].clone();
        }

        String::new()
    }

    pub fn lookups(&self, names: Vec<String>) ->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for name in &names {
            let indexes = self.lookup(name.clone());

            // Iterate through the indexes and add them to result if they haven't been added already
            for &index in &indexes {
                if !result.contains(&index) {
                    result.push(index);
                }
            }
        }

        result
    }
    pub fn lookup(&self, mut name: String) ->Vec<usize>
    {
        name = name.trim().to_string();

        let mut result = Vec::<usize>::new();

        for i   in 0..=self.names.len() -1
        {
            if self.exists(&name) {
                result.push(i );
            }
        }
        result
    }
}


#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub enum LaunchInfo
{
    Address
    {
        address : String
    },
    Name
    {
        name: String
    },
    CustomCommand{
        command : String,
        args : Vec<String>,
    },
    #[default]
    CantLaunch
}

impl LaunchInfo{
    pub fn reset(&mut self)
    {
        *self = CantLaunch
    }
    pub fn set(&mut self, launch_info: LaunchInfo)->bool
    {
        match self
        {
            CantLaunch => {
                *self = launch_info;
                return true;
            },
            _ => false
        }
    }
    pub fn get_launch_info(&self)->LaunchInfo
    {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct App
{
    alias : Option<String>,
    groups: Names,
    process_name : String,
    launch_info : LaunchInfo,
}


impl App
{
    pub fn new(launch_info : LaunchInfo, process_name : String, alias : Option<String>) -> Self
    {
        Self
        {
            alias,
            process_name,
            launch_info,
            groups: Names::new(),
        }
    }


    pub fn set_alias(&mut self, input : String)
    {
        match &mut self.alias
        {
            Some(alias)=>alias.clear(),
            None =>{},
        }

        self.alias = Some(input);
    }
    pub fn get_alias(&self)->Option<String>
    {
        self.alias.clone()
    }

    // sets name property using get_name
    pub fn set_process_name(&mut self, process_name : String)
    {
        self.process_name = process_name
    }
    pub fn get_process_name(& self)->String{
        self.process_name.clone()
    }

    pub fn set_launch_info(&mut self, launch_info : LaunchInfo)->bool
    {
        self.launch_info.set(launch_info)
    }
    pub fn get_launch_info(self)->LaunchInfo
    {
        self.launch_info
    }

    pub fn get_groups(&self)->Names
    {
        self.groups.clone()
    }
    pub fn add_group(&mut self, name : String)->bool
    {
        self.groups.add(name)
    }
    pub fn add_groups(&mut self, names : Vec<String>)->i32
    {
        self.groups.adds(names)
    }
    pub fn search_groups(& self, name : &String)->usize
    {
        self.groups.search(name)
    }

    pub fn run(&self)->bool
    {
        let mut command = Command::new("cmd").arg("/C");

        match self.launch_info.get_launch_info()
        {
            LaunchInfo::Address {address : an_address} => command.arg(an_address),
            LaunchInfo::Name { name : a_name} => command.arg(a_name),
            LaunchInfo::CustomCommand {command : a_command, args : some_args} => {
                    command.arg(a_command);

                    if !some_args.is_empty() {
                        command.args(some_args);
                    }
            },
            CantLaunch => return false,
        }

        App::redirect_output(&mut command, None);

        self.command_confirm(&command.spawn(), "runn")
    }




    pub fn kill(&self)
    {

        let command : &mut Command = &mut Command::new("taskkill");

        command.arg("/F");
        command.arg("/IM");
        command.arg(&self.process_name);

        App::redirect_output(command, None);

        let child = command.spawn();

        self.command_confirm(&child, "kill");

        let _ = child.expect("").wait();

    }


    pub fn restart(&self)
    {
        if App::is_app_alive(self.process_name.as_str()) {
            self.kill();}

        self.run();
    }

    pub fn action(&mut self, action : &str)->bool
    {
        match action
        {
            "kill" => {
                self.kill();
                return true;},
            "run" => {
                self.run();
                return true;},
            "restart" => {
                self.restart();
                return true;
            },
            _ => return false,
        }
    }

    fn is_app_alive(process_name: &str) -> bool {
        // Use the `tasklist` command on Windows to list running processes.
        let tasklist_output = Command::new("tasklist")
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output());

        match tasklist_output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains(process_name)
            }
            Err(_) => false, // Failed to run the command or read the output.
        }
    }
    fn command_confirm(&self, child : &io::Result<Child>, action : &str)->bool
    {
        match child {

            Ok(_) => {
                // The application is now running.
                match &self.alias
                {
                    Some(alias) => println!("{}ig {}",action , alias),
                    None => println!("{}ig {}",action , self.process_name),
                }

                return true;
            }
            Err(err) => {

                match &self.alias
                {
                    Some(alias) => println!("problem {}ig {}",action , alias),
                    None => println!("problem {}ig {} : {}",action , self.process_name, err),

                }

                util::get_key();
                false
            }
        }
    }
    fn redirect_output(command :&mut Command, loc : Option<&str>)
    {
        match loc {
            None => {
                command.stdout(Stdio::null());
                command.stderr(Stdio::null());
            }
            Some(_) => {

            }
        }
    }

}



