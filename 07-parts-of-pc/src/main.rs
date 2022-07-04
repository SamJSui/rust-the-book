use std::io;
use crate::pc::case::Machine;
use parts_of_pc::motherboard_print;

pub mod pc;

fn main() {
    let mut input = String::new();
    println!("Name your Computer!");
    std::io::stdin().read_line(&mut input).expect("Did not read input");
    let input = input.trim(); // Remove newline character from input

    let mut _mach = Machine { name: String::from(input) };
    _mach.pc_print();

    motherboard_print();
}
