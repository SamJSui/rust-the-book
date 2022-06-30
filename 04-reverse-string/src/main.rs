use std::io;
use std::str;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read in line"); 
    let _reversed = String::from(reverse_string(&input));
    println!("{}", _reversed);
}

fn reverse_string (_s: &str) -> String {
    let rev = _s.chars().rev().collect::<String>();
    let bytes = rev.as_bytes();
    for (_i, &_item) in bytes.iter().enumerate() {
        if _item == b' ' {
            return (&rev[0.._i]).to_string();
        }
    }

    (&rev[..]).to_string()
}
