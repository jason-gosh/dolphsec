#![allow(unused)]

use chrono::{Datelike, TimeZone, Timelike};
use cpu_time::ProcessTime;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::debug_vars;

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

pub fn generate_numbers_from_local_calendar() -> [i128; 2]{
    let local_timestamp = chrono::Local::now();
    let mut add_local_calendar: i128 = i128::MAX;
    let mut sub_local_calendar: i128 = i128::MIN;

    let local_timestamp_str = format!(
        "{}{:02}{:02}{}{}{}{}", 
        local_timestamp.year(), 
        local_timestamp.month(), 
        local_timestamp.day(),
        local_timestamp.hour(),
        local_timestamp.minute(),
        local_timestamp.second(),
        local_timestamp.nanosecond()
    );    

    add_local_calendar = local_timestamp_str.parse::<i128>().unwrap_or(i128::MAX);

    sub_local_calendar =  (local_timestamp.year() as i128 + local_timestamp.month() as i128 + local_timestamp.day() as i128) -
        (local_timestamp.hour() as i128 + local_timestamp.minute() as i128 + local_timestamp.second() as i128 + local_timestamp.nanosecond() as i128);
    
    if cfg!(debug_assertions) {
        debug_vars!(local_timestamp, local_timestamp_str, add_local_calendar, sub_local_calendar);
    }
    [add_local_calendar, sub_local_calendar]
}

pub fn generate_numbers_from_hostname(salt: u16) -> u64 {
    let hostname: String = match hostname::get() {
        Ok(name) => name.to_string_lossy().into_owned(),
        Err(_) => "unknown_host".to_string(), 
    };
    
    let mut hasher = DefaultHasher::new();
    if salt > 0 {
        for _ in 0..salt {
            hostname.hash(&mut hasher);
        }
    }
    hostname.hash(&mut hasher);
    let hostname_hash = hasher.finish();
    
    if cfg!(debug_assertions) {
        debug_vars!(hostname, hostname_hash);
    }
    hostname_hash
}
