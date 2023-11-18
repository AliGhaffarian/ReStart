use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::back_utils::utils::is_index_inbound;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CustomOrder{
    order : Vec<usize>
}


impl CustomOrder{
    pub fn new() -> Self{
        Self {
            order : Vec::<usize>::new()
        }
    }
    pub fn from(order : Vec<usize>)->Self{
        if Self::is_valid_order(&order) == false{
            panic!("tried to make from invalid order")
        }
        Self{
            order
        }
    }
    pub fn len(&self)->usize{
        self.order.len()
    }
    pub fn swap(&mut self, first_value : usize , second_value : usize)->bool{

        if is_index_inbound(&self.order, first_value) == false
            || is_index_inbound(&self.order, second_value) == false{
            panic!("\ntried to swap invalid indexes\nindexes:{} and {}\norder length: {}", first_value, second_value, self.order.len())
        }

        let first_index = self.search(first_value)
            .expect(format!("\nindex is in bound but not found in order\nindex:{}, order{:?}", first_value, self.order).as_str());
        let second_index = self.search(second_value)
            .expect(format!("\nindex is in bound but not found in order\nindex:{}, order{:?}", second_value, self.order).as_str());

        self.order.swap(first_index, second_index);

        true
    }
    pub fn push(&mut self, value : usize){
        self.insert(value , self.order.len())
    }

    //panic if try to remove a none existing value
    pub fn remove(&mut self, value : usize){
        let index = self.search(value).expect(
            format!("\ntried to search for a non_existing value in custom order\nvalue:{}\norder length:{}", value , self.order.len()
            ).as_str());

        self.decrease_all_more_valuables_or_eq(value);
        self.order.remove(index);
    }
    pub fn is_empty(&self)->bool{
        self.order.is_empty()
    }
    pub fn insert(&mut self , index : usize, value : usize){
        self.increase_all_more_valuables_or_eq(value);
        self.order.insert(index, value);
    }
    fn increase_all_more_valuables_or_eq(&mut self, value : usize){
        for i in 0..=self.order.len()-1{
            if self.order[i] >= value{
                self.order[i] += 1;
            }
        }
    }

    fn decrease_all_more_valuables_or_eq(&mut self, value : usize){
        for i in 0..=self.order.len()-1{
            if self.order[i] >= value{
                self.order[i] -= 1;
            }
        }
    }
    fn search(&self, value : usize)->Option<usize>{
        for i in 0..=self.order.len() - 1{
            if self.order[i] == value{
                return Some(i)
            }
        }
        None
    }
    pub fn get_all(&self)->Vec<usize>{
        self.order.clone()
    }
    pub fn index_to_value(&self, index: usize) ->usize{
        if is_index_inbound(&self.order, index) == false{
            panic!("out of bound index ({})tried to access in order with length {}", index, self.order.len())
        }
        self.order[index]
    }
    pub fn is_valid_order(order : &Vec<usize>)->bool{

        let min = match order.iter().min() {
            Some(value)=>value,
            None=>return false
        };

        if min != &0usize {
            return false
        }

        let max = order.iter().max().unwrap();

        if max != &(order.len() - 1){
            return false
        }

        for i in 0..=*max{
            if order.iter()
                .find(|element| element.clone() == &i).is_none(){
                return false;
            }
        }
        true
    }
}