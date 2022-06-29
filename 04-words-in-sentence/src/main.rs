use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new(); // User Input
    let num_word = 0; // Word Count

    // Read User Input
    io::stdin().read_line(&mut input).expect("Failed to read in line"); 
    
    let _max_char = input.len()-1;

    loop {
        let _idx = 0; // Char Idx
        match _idx.cmp(&_max_char){
            Ordering::Equal => {
                break;
            }
            _ => {
                let (num_word, word, _idx) = find_word(num_word, &input, _idx);
                println!("{}. {}", num_word, word);
                println!("Idx: {} Max Char: {}", _idx, _max_char);
                continue;
            }
        }
    }
}

fn find_word(num_word: u64, _s: &str, idx: usize) -> (u64, &str, usize){
    let bytes = _s.as_bytes();
    let bytes = &bytes[idx..];

    for i in idx.._s.len() {
        if bytes[i] == b' ' || bytes[i] == b'\n' {
            println!("Space or newline found");
            return (num_word+1, &_s[idx..i], idx+i);
        }
    }
    
    (num_word, &_s[..], _s.len())
}
