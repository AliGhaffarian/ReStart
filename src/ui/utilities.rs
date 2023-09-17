
pub struct util{}
impl util
{
    pub fn is_all_digits(input: & String) -> bool {
        let trimmed_input = input.trim();
        trimmed_input.chars().all(|c| c.is_digit(10))
    }
}