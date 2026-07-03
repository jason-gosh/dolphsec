use std::fmt::{Debug, Formatter, Result};
use crate::modules::nums_calculator::{generate_numbers_from_cputime, generate_numbers_from_hostname, generate_numbers_from_local_calendar};
use crate::debug_vars;

#[derive(Clone)]
pub struct Key{
    name: String,
    content: String,
    size: usize
}

impl Key {
    pub fn new(name: &str, content: &str, size: &usize) -> Self {
        Self {
            name: name.to_string(),
            content: content.to_string(),
            size: size.to_owned()
        }
    }

    fn content(&mut self, new_content: String){
        self.content = new_content;
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
            .finish()
    }
}

// we should merge, the Universal timestamp with zone and try concat real location also hostname and concat cache space with random number
pub fn generate_single_key () -> Key {
    generate_base_key("single_key".to_string())

}

pub fn generate_half_key () -> Key {
    let mut key = generate_base_key("half_key".to_string());
    let mut key_content : String = key.get_value().to_string();
    let [add, sub] = generate_numbers_from_local_calendar();
    key_content.push_str(&format!("{}{}", add, sub));
    key.content(key_content);
    
    key
}

fn generate_base_key(key_name: String) -> Key {
    let [add, sub] = generate_numbers_from_cputime();
    let mut sub_str = sub.to_string();
    sub_str.truncate(5);
    let sub_cut: u16 = sub_str.parse().unwrap_or(u16::MAX);
    let hashed_hostname = generate_numbers_from_hostname(sub_cut);
    
    let base_key_str: String = format!("{}{}{}", hashed_hostname, add, sub);
    let base_key  = Key { name: key_name, content: (base_key_str.clone()), size: (base_key_str.len()) };
    if cfg!(debug_assertions) {
        debug_vars!(base_key_str, hashed_hostname);
    }
    base_key
}