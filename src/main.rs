use serde::Deserialize;




#[derive(Deserialize)]
struct App
{
    name : Option<String>,
    address : String,
    groups : Option<Vec<String>>,
}

impl App
{
    fn get_name(&self)->String
    {
        self.address.rsplit("\\").next ().unwrap ().to_string()
    }
    fn set_name(mut self)
    {
        self.name = Some(self.get_name());
    }
    fn reg_app(adress: String, apps : &mut Vec<App> ) -> bool
    {
        true
    }
    fn group_app()
    {

    }


}

fn main() {
    let apps = Vec::<App>::new();

    println!("success");

    let path = "C:\\Users\\hami\\Documents\\src\\rust\\program restarter\\not staged\\test programs\\2.bat".to_string();
    let mut an_app = App{
        address : path,
        name : None,
        groups : None
    };

    println!("{}", an_app.get_name().to_string());
}
