use crate::modules::dolph_internal_keyg::*;

mod modules;
mod macros;

fn main() {
    let single_key_out = generate_single_key();
    println!("single_key -> {:?}", single_key_out);
    println!("single_key -> {:?}", single_key_out.get_value());
}
