
use std::process::Output;
use std::io;

use serde::{Serialize, Deserialize};
//use serde_json::Result;
use std::process::{Child, Command, Stdio};

use crate::utilities::util;


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
        let to_lower_name = name.trim().to_lowercase();

        for name_in_self in &self.names
        {
            if to_lower_name.eq(&name_in_self.to_lowercase()) {
                return true;
            }
        }
        false
    }

    //returns amount of failed pushes already existing causes failing to push
    pub fn adds(&mut self, names : Vec<String>) -> i32
    {
        let mut failed_pushes = 0;

        for name in names
        {
            if self.add(name) == false{
                failed_pushes += 1;
            }
        }
        return failed_pushes;
    }
    pub fn add(&mut self, mut name: String) -> bool
    {
        name = name.trim().to_lowercase();

        return match self.exists(&name){
            false =>{
                self.names.push(name);
                true
            },
            true => true
        }
    }
    //returns amount of failed removes
    pub fn rems(&mut self, names: Vec<String>) -> i32 {

        let mut failed_removes = 0;

        for name in names {

            if self.rem(name) == false{
                failed_removes += 1
            }
        }
        failed_removes
    }

    //true if deletes
    pub fn rem(&mut self, mut name: String) -> bool {

        name = name.trim().to_lowercase();

        return match self.search(&name) {
            usize::MAX => false,
            index => {
                self.names.remove(index);
                true
            },
        };
    }

    pub fn search(&self, name: &String) ->usize
    {
        let to_lower_name = name.trim().to_lowercase();

        match self.names.iter().position(|n| n.eq(&to_lower_name)){
            Some(index)=>return index,
            None => usize::MAX,
        }
    }

    pub fn searches(&self, names: Vec<String>) ->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for name in &names {
            let index = self.search(name);

            if !result.contains(&index) {
                result.push(index);
            }
        }
        result
    }

    pub fn clear(&mut self)
    {
        self.names.clear();
    }

    pub fn get_all(&self)->Vec<String>
    {
        self.names.clone()
    }

    pub fn get(&self, index : &usize)->Option<String>
    {
        return match *index < self.names.len() {
            true => Some(self.names[*index].clone()),
            false => None,
        }
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
        *self = LaunchInfo::CantLaunch
    }
    pub fn set(&mut self, launch_info: LaunchInfo)->bool
    {
        match self
        {
            LaunchInfo::CantLaunch => {
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
    pub fn new( input_launch_info : LaunchInfo,  input_process_name : String,  input_alias : Option<String>) -> Self
    {
        Self
        {
            alias : input_alias,
            process_name : input_process_name,
            launch_info : input_launch_info,
            groups: Names::new(),
        }
    }

    pub fn set_alias(&mut self, mut input : String)
    {
        input = input.trim().to_lowercase();

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
        self.process_name = process_name.trim().to_lowercase()
    }
    pub fn get_process_name(& self)->String{
        self.process_name.clone()
    }

    pub fn reset_launch_info(&mut self){
        self.launch_info.reset()
    }
    pub fn set_launch_info(&mut self, launch_info : LaunchInfo)->bool
    {
        self.launch_info.set(launch_info)
    }
    pub fn get_launch_info(&self)->LaunchInfo
    {
        self.launch_info.clone()
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
    pub fn rem_groups(&mut self, names : Vec<String>)->i32 {
        self.groups.rems(names)
    }

    pub fn rem_group(&mut self, name : String)->bool{
        self.groups.rem(name)
    }

    pub fn search_group(& self, name : &String)->usize
    {
        self.groups.search(name)
    }
    pub fn search_groups(& self, names : Vec<String>)->Vec<usize>{
        self.groups.searches(names)
    }
    pub fn exists_group(& self, name : &String)->bool{
        self.groups.exists(name)
    }


    pub fn run_windows(&self)->bool{

        let mut binding = Command::new("cmd");
        let mut command = binding.arg("/C");

        command = match self.launch_info.get_launch_info()
        {
            LaunchInfo::Address {address : an_address} => command.arg(an_address),
            LaunchInfo::Name { name : a_name} => command.arg(a_name),
            LaunchInfo::CustomCommand {command : a_command, args : some_args} => {
                command.arg(a_command);

                if !some_args.is_empty() {
                    command.args(some_args);
                }
                // Clone the command before passing it
                command
            },
            LaunchInfo::CantLaunch => return false,
        };

        App::redirect_output(&mut command, None);

        self.command_confirm(&command.spawn(), "runn")
    }
    pub fn run_linux(&self)->bool{

        let mut command : Command;

        command = match self.launch_info.get_launch_info()
        {
            LaunchInfo::Address {address : an_address} => Command::new(an_address),
            LaunchInfo::Name { name : a_name} => Command::new(a_name),
            LaunchInfo::CustomCommand {command : a_command, args : some_args} => {
                
                let mut a_command = Command::new(a_command);

                if !some_args.is_empty() {
                    a_command.args(some_args);
                }
                // Clone the command before passing it
                a_command
            },
            LaunchInfo::CantLaunch => return false,
        };

        App::redirect_output(&mut command, None);

        self.command_confirm(&command.spawn(), "runn")
    }

    pub fn run(&self)->bool
    {    
        #[cfg(target_os = "linux")]{
            self.run_linux()
        }
        #[cfg(target_os = "windows")]{
            self.run_windows
        }
    }

    pub fn kill_windwos(&self){
        let command : &mut Command = &mut Command::new("taskkill");

        command.arg("/F");
        command.arg("/IM");
        command.arg(&self.process_name);

        App::redirect_output(command, None);

        let child = command.spawn();

        self.command_confirm(&child, "kill");

        let _ = child.expect("").wait();
    }

    pub fn get_pid(&self)->Option<String> {

        return match Command::new("pgrep").arg(self.process_name.clone()).output(){
            Ok(output)=>{
                Some(String::from_utf8(output.stdout).unwrap())
            },
            Error => None,
        }
    }

    pub fn kill_linux(&self){

        let mut binding = Command::new("kill");
        let mut command =binding.arg("-TERM");

        match self.get_pid(){
            Some(pid)=> command = command.arg(pid.trim()),
            None => return
        }

        //App::redirect_output(command, None);

        let child = command.spawn();

        self.command_confirm(&child, "kill");

        let _ = child.expect("").wait();

    }

    pub fn kill(&self)
    {

        #[cfg(target_os = "linux")]{
            self.kill_linux()
        }
        #[cfg(target_os = "windows")]{
            self.kill_windows
        }

    }


    pub fn restart(&self)
    {
        if App::is_app_alive(self.process_name.as_str()) {
            self.kill();}

        self.run();
    }

    pub fn action(&mut self, action : &str)->bool
    {
        return match action
        {
            "kill" => {
                self.kill();
                true},
            "run" => {
                self.run();
                true},
            "restart" => {
                self.restart();
                true
            },
            _ => false,
        }
    }

    pub fn is_app_alive_windows(process_name : &str) -> bool{
        // Use the `tasklist` command on Windows to list running processes.
        let tasklist_output = Command::new("tasklist")
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output());

        match tasklist_output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout).to_lowercase();
                let process_name_lower = process_name.to_lowercase();
                output_str.contains(&process_name_lower)
            }
            Err(_) => false, // Failed to run the command or read the output.
        }
    }
    pub fn is_app_alive_linux(process_name : &str) -> bool{
        
        let output = Command::new("pgrep")
            .arg(process_name)
            .output()
            .expect("Failed to execute pgrep");
        
        

        output.status.success()
    }


    pub fn is_app_alive(process_name: &str) -> bool {
        #[cfg(target_os = "windows")]{
            Self::is_app_alive_windows(process_name)
        }
        #[cfg(target_os = "linux")]{
            Self::is_app_alive_linux(process_name)
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



