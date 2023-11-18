use serde::{Serialize, Deserialize};
use serde_json;
use crate::appclass::App;
use crate::appclass::LaunchInfo;
use crate::custom_order::CustomOrder;
use crate::back_utils::utils::insert_sorted;
use crate::back_utils::utils::is_index_inbound;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AppList
{
    pub apps : Vec<App>,
    pub custom_order : CustomOrder
}

impl AppList
{

    pub fn new()->Self{
        Self{
            apps : Vec::<App>::new(),
            custom_order : CustomOrder::new()
        }
    }

    pub fn from(mut input_apps: Vec<App>, custom_order : Option<CustomOrder>) -> Self
    {
        //checking if there's a given custom order
        let order =
            if custom_order.is_none()
                   //custom order is invalid for the given applist
                || custom_order.unwrap().len() != input_apps.len()
                //panicking
                {panic!("invalid order for the given applist at creation time")}
            //custom order is OK
            else{custom_order.unwrap()};

        //applist must be sorted cuz it will use binary search
        input_apps.sort();
        Self {
            custom_order: order,
            apps: input_apps,
        }
    }

    pub fn replace_custom_order(&mut self, custom_order : CustomOrder){
        if custom_order.unwrap().len() != self.apps.len(){
            panic!("tried to replace an invalid custom order for applist")
        }

        self.custom_order = custom_order;
    }
    pub fn apps_binary_search_process_name_based(&self, process_name: String) -> Option<usize> {
        return match self.apps.binary_search_by(|app| app.get_process_name().cmp(&process_name)){
            Ok(index)=>Some(index),
            Err(_)=>None
        }
    }

    pub fn insert_app_sorted(&mut self, app: App){
        //inset app and custom pos
        if let Ok(index) = insert_sorted(&mut self.apps, app, true){
            self.custom_order.push(index)
        }
        else{
            panic!("duplicate element tried to insert")
        }
    }

    //set launch info by index false if index is out of bound
    pub fn set_app_launch_info_index(&mut self, index: usize, launch_info: LaunchInfo, is_custom_ordered : bool) -> bool
    {
        let index = self.to_ordered_index(index, is_custom_ordered);
        self.apps[index].set_launch_info(launch_info)
    }
    //translates given index to ordered
    pub fn to_ordered_index(&self, index : usize, custom_ordered : bool)->usize{
        if custom_ordered{
            return self.custom_order.index_to_value(index);
        }
        if is_index_inbound(&self.apps, index) == false{
            panic!("tried to use invalid index");
        }
        index
    }
    //set app alias by index false if index is out of bound
    pub fn set_app_alias_index(&mut self, index: usize, input: &String, is_custom_ordered : bool) {
        let index = self.to_ordered_index(index, is_custom_ordered);
        self.apps[index].set_alias(input.clone());
    }

    //set app process name by index false if index is out of bound
    pub fn set_app_process_name_index(&mut self, index: usize, input: String, is_custom_ordered : bool) {
        let index = self.to_ordered_index(index, is_custom_ordered);
        self.apps[index].set_process_name(input.clone());

        if self.is_duplicate_process_name(input){
            panic!("tried to register set duplicate process names")
        }
    }

    fn is_duplicate_process_name(&self, process_name : String)->bool{
        if let Some(_) = self.apps_binary_search_process_name_based(process_name){
            true
        }
        else{
            false
        }
    }

    //core of running apps if group isn't None checks for working directory in there
    pub fn app_index_action(&mut self, index: usize, action: &str, working_directory: Option<String>, is_custom_ordered : bool) -> bool
    {
        self.apps[
            self.to_ordered_index(index, is_custom_ordered)]
            .action(action, working_directory)
    }


    //removes app
    //note : remove from custom order
    pub fn remove_app_by_process_name(&mut self, app_process_name: &String) -> bool
    {
        return match self.apps_binary_search_process_name_based(app_process_name.to_owned()) {
            None => false,

            Some(index) => {
                self.apps.remove(index);
                self.custom_order.remove(index);
                true
            },
        }
    }

    pub fn print_all(&self, print_data: AppListPrintData) {
        if self.apps.len() == 0{
            print!("empty applist ! ");
            return
        }

        let order: Vec<usize>;
        //declaring print order
        if print_data.custom_ordered == true {
            order = self.custom_order.get_all();
        } else {
            order = (0..=self.apps.len() - 1).collect();
        }
        //end of declaring print order

        //checking detail option
        match print_data.detail_options {
            DetailApproach::FullDetail => {
                for i in order{
                    self.apps[i].print(0, true);
                }
            },
            DetailApproach::Name {alias, separate_string}=>{
                for i in order{
                    let name;
                    if alias == true {
                        name = self.apps[i].get_alias().unwrap_or(self.apps[i].get_process_name());
                    }
                    else {
                        name = self.apps[i].get_process_name();
                    }
                    println!("{} {}", name, separate_string);

                }
            },
        }
    }
}
#[derive(Clone)]
pub enum DetailApproach {
    FullDetail,
    Name{
        alias : bool,
        separate_string : String,
    }
}
#[derive(Clone)]
pub struct AppListPrintData{
    pub detail_options : DetailApproach,
    pub prefix_options : Option<String>,
    pub custom_ordered : bool,
}
