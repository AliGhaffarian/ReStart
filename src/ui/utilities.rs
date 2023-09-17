use std::io;
pub struct util{}
impl util
{
    pub fn is_all_digits(input: & String) -> bool {
        let trimmed_input = input.trim();
        trimmed_input.chars().all(|c| c.is_digit(10))
    }
    pub fn get_key() {
        println!("press Enter to continue");
        let mut input= "".to_string();
        let _ = io::stdin().read_line(&mut input).unwrap();
    }
}