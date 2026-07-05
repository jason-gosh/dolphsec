use std::fmt::{Debug, Formatter, Result};
use crate::modules::nums_calculator::{generate_numbers_from_cputime, generate_numbers_from_hostname, generate_numbers_from_local_calendar};
use crate::{debug_vars, measure};

#[derive(Clone)]
pub struct Key{
    name: String,
    content: String,
    size: usize,
    lenght: usize
}

impl Key {
    pub fn new(name: String, content: String) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            size: content.len(),
            lenght: content.chars().count()
        }
    }

    fn name(&mut self, new_name: String){
        self.name = new_name;
    }
    
    fn content(&mut self, new_content: String){
        self.content = new_content.clone();
        self.size = new_content.len();
        self.lenght = new_content.chars().count();
    }

    fn push_content(&mut self, additional_content: String){
        self.content += &additional_content;
        self.lenght += additional_content.chars().count();
        self.size = self.content.len();
    }

    fn try_operate(&mut self){
        //prevents process if none
        if !self.content.contains("-") && !self.content.contains("*"){
          return;   
        } 
        
        let sub_content = &self.content.split_once("-");
        match sub_content {
            Some((l_con, r_con)) => {
                let mut r_new_cont: i128 = 1; 
                for s in r_con.split('*') {
                    let Ok(number) = s.trim().parse::<i128>() else {
                        return; 
                    };
                    
                    r_new_cont = r_new_cont.saturating_mul(number);
                }
        
                let mut new_content = l_con.to_string();
                debug_vars!(new_content);
                new_content.push_str(&r_new_cont.to_string());
                self.content = new_content;
            }
            None => {
                return;
            }
        }            
    }
}

pub trait KeyBehavior{
    fn get_value(&self) -> &str;
}

impl KeyBehavior for Key{
    fn get_value(&self) -> &str {
        &self.content 
    }
}

impl Debug for Key{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("key")
            .field("name", &self.name)
            .field("content", &self.content)
            .field("size", &self.size)
            .field("length", &self.lenght)
            .finish()
    }
}

pub fn generate_single_key () -> Key {
    let single_key = generate_base_key("single_key".to_string());
    
    single_key
}

pub fn generate_half_key () -> Key {
    let mut half_key = generate_base_key("half_key".to_string());
    let (_, duration_elapsed) = measure!({
        let mut key_content : String = half_key.get_value().to_string();
        let [add, sub] = generate_numbers_from_local_calendar();
        key_content.push_str(&format!("{}{}", add, sub));
        half_key.content(key_content)
    });
    half_key.push_content(format!("*{}", duration_elapsed.as_nanos()));
    debug_vars!(duration_elapsed, half_key);
    half_key.try_operate();

    half_key
}

pub fn generate_hight_key () -> Key {
    let mut hight_key = generate_half_key();
    hight_key.name("hight_key".to_string());

    
    debug_vars!(hight_key);
    hight_key
}

fn generate_base_key(key_name: String) -> Key {
    let [add, sub] = generate_numbers_from_cputime();
    let mut sub_str = sub.to_string();
    sub_str.truncate(5);
    let sub_cut: u16 = sub_str.parse().unwrap_or(u16::MAX);
    let hashed_hostname = generate_numbers_from_hostname(sub_cut);
    
    let base_key_str: String = format!("{}{}{}", hashed_hostname, add, sub);
    let base_key = Key::new(key_name, base_key_str.clone());
    debug_vars!(base_key_str, hashed_hostname, base_key);
    
    base_key
}