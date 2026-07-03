use crate::modules::dolph_internal_keyg::*;

mod modules;
mod macros;

fn main() {
    let single_key_out = generate_single_key();
    let half_key_out = generate_half_key();
    println!("single_key_out -> {:?}", single_key_out);
    println!("half_key_out -> {:?}", half_key_out);
}
