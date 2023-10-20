use std::cell::RefCell;
use std::string::ToString;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;
use std::collections::HashMap;
use std::rc::Rc;
use crate::utilities::Util;
use crate::groups::Group;
use crate::appclass::App;
use crate::appclass::LaunchInfo;



#[derive(Clone, Default)]
pub struct AppList
{
    pub apps : Vec<Rc<RefCell<App>>>,
    pub custom_order : Option<Vec<usize>>
}

impl AppList
{
    pub fn new (input_apps : Vec<Rc<RefCell<App>>>, input_custom_order : Option<Vec<usize>>) -> Self
    {
        Self{
            custom_order : input_custom_order,
            apps : input_apps,
        }
    }


    pub fn apps_binary_search_process_name_based(apps : &Vec<Rc<App>>, process_name : String)->Result<usize, usize>{
        todo!()
    }

    pub fn insert_app(&mut self, app : App, sorted : bool)->bool{
        //inset app and custom pos
        todo!()
    }

    //set launch info by index false if index is out of bound
    pub fn set_app_launch_info_index(&mut self, index : usize , launch_info : LaunchInfo)->bool
    {
        return match self.is_index_inbound(index) {
            true => self.apps[index].borrow_mut().set_launch_info(launch_info),
            false => false,
        }
    }

    //set app alias by index false if index is out of bound
    pub fn set_app_alias_index(&mut self, index : usize , input : &String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps[index].borrow_mut().set_alias(input.clone());
                true
            },
            false => false,
        }
    }

    //set app process name by index false if index is out of bound
    pub fn set_app_process_name_index(&mut self, index : usize , input : String)->bool {
        return match self.is_index_inbound(index) {
            true => {
                self.apps[index].borrow_mut().set_process_name(input);
                true
            },
            false => false,
        }
    }
    pub fn search_process_name(&self, name : &String) -> Option<usize>
    {
        todo!()
    }

    pub fn apps_len(&self) ->usize
    {
        self.apps.len()
    }

    pub fn is_index_inbound(&self, index : usize)->bool
    {
        index < self.apps_len()
    }

    //core of running apps if group isn't None checks for working directory in there
    pub fn app_index_action(&mut self, index : usize, action : &str, working_directory: Option<String>) ->bool
    {
        return match self.is_index_inbound(index){
            true => {
                self.apps[index].borrow().action(action, working_directory)
            },
            false => false
        }/*match self.is_index_inbound*/
    }


    //removes app index from all groups before removing
    pub fn remove_app_by_process_name(&mut self, app_process_name : &String) ->bool
    {
        return match self.search_process_name(app_process_name) {
            None => false,

            Some(index) => {
                self.apps.remove(index);
                true
            },
        }
    }




    //swaps the two app index from all groups and in apps
    pub fn swap(&mut self, first_index : usize , second_index : usize)->bool{
        todo!()
    }

    pub fn print_all(&self, print_data : &AppListPrintData){

        let order : Vec<usize>;
        let prefix = print_data.prefix.clone().unwrap_or("".to_string());
        if print_data.custom_ordered == true{
            order = match self.clone().custom_order{
                Some(order)=>order,
                None =>{
                    println!("Err : unexpected custom order print... \nprint will continue with default ordering.");
                    Util::get_key();
                    (0..=self.apps.len() - 1).collect()
                }
            }
        }
        else{
            order = (0..=self.apps.len() - 1).collect();
        }


        match &print_data.option{
            AppListPrintOptions::FullDetail{numbered} =>{
                for i in order{
                    println!("{}",prefix);
                    self.apps[i].borrow().print(numbered.clone());
                    println!("\n");
                }

                return
            }

            AppListPrintOptions::ProcessName{print_approach} => {

                if print_data.repeat_prefix_for_each_line == false{
                    println!("{}", prefix);
                }

                for i in order{
                    if print_data.repeat_prefix_for_each_line == true{
                        print!("{}", prefix);
                    }
                    print!("{}", self.apps[i].borrow().get_process_name());
                    match print_approach {
                        PrintApproach::MultiLine => {println!()}
                        PrintApproach::SingleLine { separate_string } => {print!("{}", separate_string)}
                    }
                }
            }
            AppListPrintOptions::Alias {print_approach}=>{
                if print_data.repeat_prefix_for_each_line == false{
                    print!("{}", prefix);
                }

                for i in order{
                    if print_data.repeat_prefix_for_each_line == true{
                        print!("{}", prefix);
                    }
                    print!("{}", self.apps[i].borrow().get_alias().unwrap_or(self.apps[i].borrow().get_process_name()));
                    match print_approach {
                        PrintApproach::MultiLine => {println!()}
                        PrintApproach::SingleLine { separate_string } => {print!("{}", separate_string)}
                    }
                }
            }
        }
    }


}

pub enum AppListPrintOptions{
    FullDetail{
        numbered : bool
    },
    ProcessName{
        print_approach : PrintApproach
    },
    Alias{
        print_approach : PrintApproach
    },
}

pub enum PrintApproach {
    MultiLine,
    SingleLine{
        separate_string : String,
    }
}

pub struct AppListPrintData{
    pub option : AppListPrintOptions,
    pub prefix : Option<String>,
    pub repeat_prefix_for_each_line : bool,
    pub custom_ordered : bool
}
