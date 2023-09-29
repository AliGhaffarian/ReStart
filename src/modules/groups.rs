use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Group {
    name : String,
    working_directory : Option<String>,
    index_of_members : Vec<usize>,
}

impl Group {
    pub fn new(name : String)->Self {
        Self{
            name : name.trim().to_string(),
            working_directory : None,
            index_of_members : Vec::<usize>::new()
        }
    }

    pub fn set_working_directory(&mut self, directory : Option<String>)->bool{
        match directory {
            None => {
                self.working_directory = None;
                true
            }
            Some(working_directory) => {
                match working_directory.trim().len(){
                    0 => return false,
                    _ => {
                        self.working_directory = Some(working_directory.trim().to_string());
                        true
                    }
                }
            }
        }
    }
    pub fn set_name(&mut self, name : String){
        self.name = name.trim().to_string();
    }

    pub fn add_member(&mut self, index : usize)->bool{
        if self.index_of_members.contains(&index){
            return false;
        }

        self.index_of_members.push(index);
        true
    }
    pub fn rem_member(&mut self, index : usize)->bool{
        if self.index_of_members.contains(&index) == false{
            return false;
        }

        self.index_of_members.remove(index);
        true
    }
    pub fn clear_members(&mut self){
        self.index_of_members.clear()
    }

    pub fn get_working_directory(& self)->Option<String>{
        self.working_directory.clone()
    }
    pub fn get_name(& self)->String{
        self.name.clone()
    }
    pub fn get_members(& self)->Vec<usize>{
        self.index_of_members.clone()
    }
    pub fn swap(&mut self, first_member: usize, second_member: usize) ->bool{

        let first_index = match self.search(first_member) {
            Some(index) => index,
            None => return false
        };
        let second_index = match self.search(second_member){
            Some(index)=>index,
            None=>return false
        };

        self.index_of_members.swap(first_index,second_index);
        true
    }

    pub fn search(& self, member: usize) ->Option<usize>{
        for index in 0..=self.index_of_members.len() - 1{
            if self.index_of_members[index] == member {
                return Some(index)
            }
        }
        None
    }

    pub fn exists_member(& self, member : usize)->bool{
        self.index_of_members.contains(&member)
    }
}
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Groups
{
    groups: Vec<Group>,
}

impl Groups {
    pub fn new()->Self
    {
        Self{
            groups: Vec::<Group>::new()
        }
    }
    pub fn exists(&self, name : &String) -> bool
    {
        let to_lower_name = name.trim().to_lowercase();

        for name_in_self in &self.groups
        {
            if to_lower_name.eq(&name_in_self.get_name().to_lowercase()) {
                return true;
            }
        }
        false
    }
    pub fn gather_groups_by_name(&self, names : Vec<String>) ->Vec<Group>{
        let indexes  = self.search_group_names(names);

        let mut result = Vec::<Group>::new();

        for index in indexes{
            result.push(self.groups[index].clone());
        }

        result
    }

    pub fn get_all_names(&self)->Vec<String>{

        let mut result = Vec::<String>::new();

        for name in &self.groups {
            result.push(name.get_name());
        }

        result
    }

    //returns amount of failed pushes already existing causes failing to push
    pub fn adds(&mut self, names : Vec<Group>) -> i32
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
    pub fn add(&mut self, name: Group) -> bool
    {
        return match self.exists(&name.get_name()){
            false =>{
                self.groups.push(name);
                true
            },
            true => true
        }
    }
    //returns amount of failed removes
    pub fn remove_groups(&mut self, group_names: Vec<String>) -> i32 {

        let mut failed_removes = 0;

        for group_name in group_names {

            if self.remove_group(group_name) == false{
                failed_removes += 1
            }
        }
        failed_removes
    }

    //true if deletes
    pub fn remove_group(&mut self, group_name: String) -> bool {

        return match self.search_group_name(&group_name) {
            None => false,
            Some(index) => {
                self.groups.remove(index);
                true
            },
        };
    }

    pub fn remove_member(&mut self, member_index : usize, group_name : String )->bool{
        match self.search_group_name(&group_name){
            Some(index) => self.groups[index].rem_member(member_index),
            None => false
        }
    }

    //removes a member from all groups
    pub fn remove_member_global(&mut self, member_index : usize){
        for group in self.groups.clone(){
            self.remove_member(member_index, group.get_name());
        }
    }

    pub fn search_group_name(&self, group_name: &String) ->Option<usize>
    {
        let to_lower_name = group_name.trim().to_lowercase();

        match self.groups.iter().position(|n| n.get_name().to_lowercase().eq(&to_lower_name)){
            Some(index)=>return Some(index),
            None => None,
        }
    }

    pub fn search_group_names(&self, group_names: Vec<String>) ->Vec<usize>
    {
        let mut result = Vec::<usize>::new();

        for name in &group_names {
            let index = self.search_group_name(name);

            match index {
                Some(index) => {
                    if !result.contains(&index) {
                    result.push(index);
                    };
                }
                None => continue
            }
        }
        result
    }

    pub fn clear(&mut self)
    {
        self.groups.clear();
    }

    pub fn get_all(&self)->Vec<Group>
    {
        self.groups.clone()
    }

    pub fn get_group_by_index(&self, index : &usize) ->Option<Group>
    {
        return match *index < self.groups.len() {
            true => Some(self.groups[*index].clone()),
            false => None,
        }
    }

    pub fn swap_members_global(&mut self, first_member_index : usize, second_member_index : usize){
        for mut group in self.groups.clone(){
            group.swap(first_member_index, second_member_index);
        }
    }

    pub fn add_member_to_group(&mut self, group_name : &String, member_index : usize)->bool{
        return match self.search_group_name(&group_name) {
            Some(index)=>{
                self.groups[index].add_member(member_index);
                return true
            },
            None => false
        }
    }

    pub fn add_member_to_groups(&mut self, groups_name : &Vec<String>, member_index : usize){
        for group in groups_name{
            self.add_member_to_group(group, member_index);
        }
    }

    pub fn get_member_group_names(& self, member : usize)->Vec<String>{
        let mut result = Vec::<String>::new();
        for group in self.groups.clone(){
            if group.exists_member(member){
                result.push(group.get_name());
            };
        }
        result
    }

    pub fn add_working_directory_to_group(&mut self, group_name : String, directory : String)->bool{

        return match self.search_group_name(&group_name){
            Some(index)=>{
                self.groups[index].set_working_directory(Some(directory));
                return true
            },
            None => false
        }

    }

    pub fn len(& self)->usize{
        self.groups.len()
    }

}
