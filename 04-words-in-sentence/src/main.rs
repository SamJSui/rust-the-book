use std::io;

fn main() {
    let mut input = String::new(); // User Input
    let mut num_word = 1; // Word Count
    let mut _idx = 0;
    // Read User Input
    io::stdin().read_line(&mut input).expect("Failed to read in line"); 
    let _input_len = input.len();

    loop {
        (num_word, _idx) = find_word(num_word, &input, _idx);
        if _idx == _input_len{
            break;
        }
    }
}

fn find_word(num_word: u64, _s: &str, idx: usize) -> (u64, usize){
    let bytes = _s.as_bytes();
    let bytes = &bytes[idx..];

    for i in idx.._s.len() {
        if bytes[i] == b' ' || bytes[i] == b'\n' {
            println!("{}. {}", num_word, &_s[idx..i]);
            return (num_word+1, idx+i+1);
        }
    }
    
    (num_word+1, _s.len())
}
