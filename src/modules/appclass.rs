

use std::io;

use serde::{Serialize, Deserialize};
//use serde_json::Result;
use std::process::{Child, Command, Stdio};

use crate::utilities::Util;

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
    pub fn run_linux(&self, directory : Option<String>) ->bool{

        let working_directory = match directory{
            Some(directory) => directory,
            None => "~".to_string()
        };

        let mut binding = Command::new("cd");
        let mut command = binding.arg(working_directory);

        let args = match self.launch_info.get_launch_info()
        {
            LaunchInfo::Address {address : an_address} => vec![an_address],
            LaunchInfo::Name { name : a_name} => vec![a_name],
            LaunchInfo::CustomCommand {command : a_command, args : some_args} => {
                
                let mut args = vec![a_command];

                if !some_args.is_empty() {
                    for arg in some_args{
                        args.push(arg);
                    }
                }
                // Clone the command before passing it
                args
            },
            LaunchInfo::CantLaunch => return false,
        };

        command.args(args);

        App::redirect_output(&mut command, None);

        self.command_confirm(&command.spawn(), "runn")
    }

    pub fn run(&self, directory : Option<String>) ->bool
    {    
        #[cfg(target_os = "linux")]{
            self.run_linux(directory)
        }
        #[cfg(target_os = "windows")]{
            self.run_windows()
        }
    }

    pub fn kill_windows(&self){
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
            _ => None,
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
            self.kill_windows()
        }

    }


    pub fn restart(&self, working_directory : Option<String>)
    {
        if App::is_app_alive(self.process_name.as_str()) {
            self.kill();}

        self.run(working_directory);
    }

    pub fn action(&mut self, action : &str, working_directory: Option<String>) ->bool
    {
        return match action
        {
            "kill" => {
                self.kill();
                true},
            "run" => {
                self.run(working_directory);
                true},
            "restart" => {
                self.restart(working_directory);
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

                Util::get_key();
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



