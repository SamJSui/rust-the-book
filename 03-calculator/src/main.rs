use std::io;
use std::io::Write;
use std::process;

// ARITHMETIC
fn add() -> i64 {
    let mut _num = String::new();
    println!("a + b = ?");

    print!("a: ");
    io::stdin().read_line(&mut _num).expect("Failed to read in line");
    let a: i64 = match _num.trim().parse() { // Input Validation
        Ok(_num) => _num,
        Err(_) => {
            println!("Please enter a valid number");
            process::exit(1);
        }
    };

    _num.clear();
    print!("b: ");
    io::stdin().read_line(&mut _num).expect("Failed to read in line");
    let b: i64 = match _num.trim().parse() { // Input Validation
        Ok(_num) => _num,
        Err(_) => {
            println!("Please enter a valid number");
            process::exit(1);
        }
    };
    
    return a + b;
}

fn subtract() -> i64 {
    2
}

// MAIN

fn main() {
    let mut _choice = String::new();
    loop {
        println!("Select:");
        println!("1) Add");
        println!("2) Subtract");
        println!("3) Multiply");
        println!("4) Divide");

        io::stdin()
            .read_line(&mut _choice)
            .expect("Failed to read in line");
        let num: i64 = match _choice.trim().parse() { // Input Validation
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                process::exit(1);
            }
        };

        match num {
            1 => {
                println!("{}", add());
            },
            2 => {
                println!("{}", subtract());
            },
            _ => {
                println!("NOO!");
            },
        }
    }
}
