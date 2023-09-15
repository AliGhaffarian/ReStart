use std::io;
use std::string::ToString;
use serde::Deserialize;
use std::process::{Child, Command, Stdio};

//need to get operating system
struct UserPrefrence
{
    restart_error: bool,
    restart_msg: bool,
    app_msg: bool,
}

#[derive(Deserialize, Clone)]
pub struct App
{
    name: String,
    address: String,
    groups: Vec<String>,
}

impl App
{
    pub fn new(address: String) -> Self
    {
        Self
        {
            address: address.clone(),
            name: App::extract_name(address.clone()),
            groups: Vec::<String>::new(),

        }
    }

    // extracts name of the app by its address
    pub fn extract_name(address: String) -> String
    {
        address.rsplit("\\").next().unwrap().to_string()
    }
    // sets name property using get_name
    pub fn set_name(&mut self)
    {
        self.name = App::extract_name(self.address.clone());
    }

    pub fn exists_group(self, group: String) -> bool
    {
        for group_in_self in self.groups
        {
            if group.eq(&group_in_self) {
                return true;
            }
        }
        false
    }

    //returns amount of failed pushes already existing causes failing to push
    pub fn add_groups(&mut self, groups: Vec<String>) -> i32
    {
        let mut failed_pushes = 0;
        for group in groups
        {
            if self.to_owned().exists_group(group.clone()) {
                failed_pushes += 1
            } else {
                self.groups.push(group)
            }
        }
        return failed_pushes;
    }
    pub fn add_group(&mut self, group: String) -> bool
    {
        if self.to_owned().exists_group(group.clone()) { return false; }

        self.groups.push(group.clone());

        true
    }
    //returns amount of failed removes
    pub fn rem_groups(&mut self, groups: Vec<String>) -> i32 {
        let mut failed_removes = 0;

        for group in groups {
            if self.clone().exists_group(group.clone()) {
                let index = self.groups.iter().position(|n| n.eq(&group));
                self.groups.remove(index.unwrap());
                failed_removes += 1;
            }
        }
        failed_removes
    }
    //true if deletes
    pub fn rem_group(&mut self, group: String) -> bool {
        if self.clone().exists_group(group.clone()) {
            let index = self.groups.iter().position(|n| n.eq(&group));
            self.groups.remove(index.unwrap());
            return true;
        }
        false
    }

    pub fn rem_groups_all(&mut self)
    {
        self.groups.clear();
    }
    pub fn set_address(&mut self, address: String)
    {
        self.address = address.clone();
        self.name = App::extract_name(address);
    }

    pub fn get_name(self)->String
    {
        self.name
    }

    pub fn get_address(self)->String
    {
        self.address
    }
    pub fn get_groups(self)->Vec<String>
    {
        self.groups
    }

    pub fn run(self)
    {
        let mut command: &mut Command = &mut Command::new(self.address.clone());

        App::redirect_output(command, None);

        self.command_confirm(command.spawn(), "runn");
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
    fn command_confirm(self, child : io::Result<Child>, action : &str)
    {
        match child {

            Ok(mut child) => {
                // The application is now running.
                println!("{}ig {}",action , self.name);
            }
            Err(err) => {
                eprintln!("Error {}ing {}: {}", action, self.name , err);
            }
        }
    }

    pub fn kill(self)
    {

        let mut command : &mut Command = &mut Command::new("taskkill");

        command.arg("/F");
        command.arg("/IM");
        command.arg(self.name.clone());

        App::redirect_output(command, None);

        self.command_confirm(command.spawn(), "kill");

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
    pub fn restart(self)
    {
        if App::is_app_alive(self.name.clone().as_str()) {
            self.clone().kill();}

        self.clone().run();
    }

    pub fn action(self, action : &str)->bool
    {
        match action
        {
            "kill" => {
                self.clone().kill();
                return true;},
            "run" => {
                self.clone().run();
                return true;},
            "restart" => {
                self.clone().restart();
                return true;
            },
            _ => return false,
        }
    }
}



