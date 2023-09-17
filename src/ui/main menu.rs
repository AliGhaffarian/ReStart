use std::io;

mod utilities;
use utilities::util;

#[path = "..\\modules\\appdb.rs"] pub mod appdb;
use appdb::AppDB;
use std::string::ToString;



pub struct UI
{
    pub app_db : AppDB,
    pub commands : Vec<String>,
    pub saved : bool,
    pub defined_groups : Vec<String>,
}

impl UI {
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

    pub fn main_menu(&mut self)
    {
        self.print_all_apps(true);

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input_vec: Vec<String> = input.split(' ').map(|s| s.to_string()).collect();

        self.input_handler(input_vec);

        //o = o.iter().skip(1).map(|&s| s).collect();

    }


    pub fn input_handler(&mut self, input_vec : Vec<String>)
    {
        self.special_command_handler(input_vec.clone());

        for input in input_vec
        {
            if util::is_all_digits(&input) {
                self.app_db.app_index_action(input.trim().parse().unwrap(), "restart");
                continue;
            }

            if self.defined_groups.contains(&input)
            {
                self.app_db.app_group_action(vec![input.clone()], "restart")
            }

            println!("{} is neither a group of number of an app", input);

        }
    }

    pub fn special_command_handler(&mut self, input_vec : Vec<String>)
    {
        match input_vec[0].as_str()
        {
            "quit" => self.quit(),
            "reg" => self.regs(input_vec.iter().skip(1).cloned().collect()),
            "save" => self.save(),
            "load" => self.load(),
            "del" => self.del_app(),
            _ =>return,
        }
    }

    pub fn quit(&mut self)
    {
        println!("quit called!");
    }
    pub fn save(&mut self)
    {
        println!("save called!");
    }
    pub fn load(&mut self)
    {
        println!("load called!");
    }

    pub fn regs(&mut self, input_vec : Vec<String>)
    {
        println!("regs called!");
    }

    pub fn reg_app(&mut self)
    {

    }
    pub fn reg_group(&mut self)
    {

    }
    pub fn del_app(&mut self)
    {
        println!("del app called!");
    }


}


