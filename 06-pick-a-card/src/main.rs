use rand::Rng;
use std::io;

#[derive(Debug)]
enum Card {
    Spades{_val: u8},
    Diamonds{_val: u8},
    Hearts{_val: u8},
    Clubs{_val: u8},
}

fn gen_val () -> u8 {
    rand::thread_rng().gen_range(1..14)
}

fn card_suit () -> Card {
    let _suit: u8 = rand::thread_rng().gen_range(0..4); 
    match _suit {
        0 => {
            Card::Spades{ _val: gen_val() }   
        }
        1 => {
            Card::Diamonds{ _val: gen_val() } 
        }
        2 => {
            Card::Hearts{ _val: gen_val() } 
        }
        _ => {
            Card::Clubs{ _val: gen_val() }
        } 
    }
}

fn print_card(_card: Card, _count: Option<u64>) {
    match _card {
        Card::Spades { _val } => {
            println!("{}. {} of Spades", _count.unwrap(), _val);
        },
        Card::Diamonds { _val } => {
            println!("{}. {} of Diamonds", _count.unwrap(), _val);
        },
        Card::Hearts { _val } => {
            println!("{}. {} of Hearts", _count.unwrap(), _val);
        },
        Card::Clubs { _val } => {
            println!("{}. {} of Clubs", _count.unwrap(), _val);
        },
    }
}

fn increment(n: Option<u64>) -> Option<u64> {
    match n {
        Some(i) => Some(i+1),
        None => None,
    }
}

fn main() {
    let mut _prompt = String::new();
    io::stdin().read_line(&mut _prompt).expect("Did not read properly");
    let num: u32 = match _prompt.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
    for i in 0..num {
        let _count: Option<u64> = Some((i+1).into());
        let _card = card_suit();
        print_card(_card, _count);
        let _count = increment(_count);
    }
}
