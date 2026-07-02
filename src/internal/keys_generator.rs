use std::fmt::{Debug, Formatter, Result};
use cpu_time::ProcessTime;
use crate::debug_vars;

pub struct key{
    name: String,
    content: String,
    size: usize
}
pub trait KeyBehavior{
    fn get_value(&self) -> &str;
}

impl KeyBehavior for key{
    fn get_value(&self) -> &str {
        &self.content 
    }
}

impl Debug for key{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("key")
            .field("name", &self.name)
            .field("content", &self.content)
            .field("size", &self.size)
            .finish()
    }
}

// we should merge, the Universal timestamp with zone and try concat real location also hostname and concat cache space with random number
pub fn generate_single_key () -> key {
    let [add, sub] = generate_numbers_from_cputime();
    let single_key_str: String = format!("{}{}", add, sub);
    let single_key  = key { name: "single_key".to_string(), content: (single_key_str.clone()), size: (single_key_str.len()) };
    
    if cfg!(debug_assertions) {
        debug_vars!(single_key_str);
    }
    single_key
}

    


pub fn generate_numbers_from_cputime() -> [u128; 2] {
    let start_cpu_time_duration = ProcessTime::now().as_duration();
    
    let sta_cputime_secs        =    start_cpu_time_duration.as_secs();
    let sta_cputime_nanos       =    start_cpu_time_duration.as_nanos();
    let sta_cputime_millis      =    start_cpu_time_duration.as_millis();

    let finish_cpu_time_duration = ProcessTime::now().as_duration();

    let fin_cputime_secs        =    finish_cpu_time_duration.as_secs();
    let fin_cputime_nanos       =    finish_cpu_time_duration.as_nanos();
    let fin_cputime_millis      =    finish_cpu_time_duration.as_millis();
    
    let add_cputime_generated: u128 = (
        (sta_cputime_secs as i128 + sta_cputime_nanos as i128+ sta_cputime_millis as i128) + 
        (fin_cputime_secs as i128 + fin_cputime_nanos as i128+ fin_cputime_millis as i128)
    ).abs() as u128;
    
    let sub_cputime_generated: u128 = (
        (fin_cputime_secs as i128 + fin_cputime_nanos as i128 + fin_cputime_millis as i128) -
        (sta_cputime_secs as i128 + sta_cputime_nanos as i128 + sta_cputime_millis as i128)
    ).abs() as u128;
    
    let sat_cputime_as_str: String = format!("{}{}", sta_cputime_secs, sta_cputime_nanos );
    let fin_cputime_as_str: String = format!("{}{}", fin_cputime_secs, fin_cputime_nanos);

    if cfg!(debug_assertions) {
        debug_vars!(start_cpu_time_duration, sat_cputime_as_str, finish_cpu_time_duration, fin_cputime_as_str, add_cputime_generated, sub_cputime_generated);
    }
    [add_cputime_generated, sub_cputime_generated]
}

