use std::io;
use std::io::Write;
use std::process;

fn operands () -> (i64, i64) {
    let mut _num = String::new();
    print!("a: ");
    io::stdout().flush().expect("Toilet clogged");
    io::stdin().read_line(&mut _num).expect("Failed to read in line");
    let _a: i64 = match _num.trim().parse() { // Input Validation
        Ok(_a) => _a,
        Err(_) => {
            println!("Please enter a valid number");
            process::exit(1);
        }
    };

    _num.clear();
    print!("b: ");
    io::stdout().flush().expect("Toilet clogged");
    io::stdin().read_line(&mut _num).expect("Failed to read in line");
    let _b: i64 = match _num.trim().parse() { // Input Validation
        Ok(_b) => _b,
        Err(_) => {
            println!("Please enter a valid number");
            process::exit(1);
        }
    };
    return (_a, _b);
}

// ARITHMETIC
fn add(a: i64, b: i64) -> i64 {
    a + b
}
fn subtract(a: i64, b: i64) -> i64 {
    a - b
}
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}
fn divide(a: i64, b: i64) -> i64 {
    a / b
}
fn remainder(a: i64, b: i64) -> i64 {
    a % b
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
            // ADD
            1 => {
                println!("\na + b = ?");
                let (a, b) = operands();
                println!("{} + {} = {}\n", a, b, add(a, b));
            },
            
            // SUBTRACT
            2 => {
                println!("\na - b = ?");
                let (a, b) = operands();
                println!("{} - {} = {}\n", a, b, subtract(a, b));
            },

            // MULTIPLY
            3 => {
                println!("\na * b = ?");
                let (a, b) = operands();
                println!("{} * {} = {}\n", a, b, multiply(a, b));
            }

            // DIVIDE
            4 => {
                println!("\na / b = ?");
                let (a, b) = operands();
                println!("{} * {} = {} r {}\n", a, b, divide(a, b), remainder(a,b));
            }
            _ => {
                main();
            },
        }
    }
}
