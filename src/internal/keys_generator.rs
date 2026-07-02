use std::fmt::{Debug, Formatter, Result};
use crate::modules::nums_calculator::{generate_numbers_from_cputime, generate_numbers_from_hostname};
use crate::debug_vars;

#[derive(Clone)]
pub struct Key{
    name: String,
    content: String,
    size: usize
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
    let [add, sub] = generate_numbers_from_cputime();
    let mut sub_str = sub.to_string();
    sub_str.truncate(5);
    let sub_cut: u16 = sub_str.parse().unwrap_or(u16::MAX);
    let hashed_hostname = generate_numbers_from_hostname(sub_cut);
    
    let single_key_str: String = format!("{}{}{}", hashed_hostname, add, sub);
    let single_key  = Key { name: "single_key".to_string(), content: (single_key_str.clone()), size: (single_key_str.len()) };
    
    
    if cfg!(debug_assertions) {
        debug_vars!(single_key_str, hashed_hostname);
    }
    single_key

}

