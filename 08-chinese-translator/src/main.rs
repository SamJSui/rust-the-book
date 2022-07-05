use std::collections::HashMap;
use std::string;

fn main() {
    let mut _input = String::new();
    std::io::stdin().read_line(&mut _input).expect("Did not read in properly");
    let num: usize = match _input.trim().parse() {
        Ok(num) => num,
        Err(_) => 11,
    };

    if num == 11 {
        let _input: String = _input.trim().to_lowercase();
        let _english = vec![String::from("hello"), 
                            String::from("how are you"),
                            String::from("where is the bathroom"),
                            String::from("can i eat food"),
        ];
        let _chinese = vec![String::from("你好"), 
                            String::from("你好吗"),
                            String::from("厕所在哪里？"),
                            String::from("我可以吃吗？"),
        ];
        let mut _phrases: HashMap<_, _> = _english.into_iter().zip(_chinese.into_iter()).collect();
        let _ans: &String = &_phrases.get(String::as_str(&_input)).unwrap();
        println!("\n{}", _ans);
    }
    else {
        let _numbers = vec!['零','一','二','三','四','五','六','七','八','九','十'];
        let _ans: char = *_numbers.get(num).unwrap();
        println!("\n{}", _ans);
    }
}
