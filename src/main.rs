use std::{collections::{HashMap, HashSet}};
use crate::modules::dolph_internal_keyg::*;

mod modules;
mod macros;

fn main() {
   
    let mut seen_keys: HashSet<Key> = HashSet::new(); 
    seen_keys.reserve(1_000_000);

    for i in 1..=100_000_000 {
   
        let single_key = generate_half_key();
        // Si el valor ya existía, devuelve 'false' de forma instantánea.
   
        if !seen_keys.insert(single_key.clone()) {
            print!("llave repetida: {:?} iteración: {}", single_key, i);
            panic!("Se detuvo la ejecución por llave duplicada.");
        }
   
        if i % 100_000 == 0 {
            println!("iteración número: {} - TAMAÑO LISTA: {}", i, seen_keys.len());
        }
   
    }
    
    println!("--- FIN DEL CICLO  ---");
    println!("Se finalizo el ciclo de vida del loop iteración: {}", seen_keys.len());
}
