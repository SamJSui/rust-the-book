use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write, BufRead, BufReader};

fn main() {
    let _file = File::open("file.txt").unwrap_or_else(|error|{ // Opens file.txt
        if error.kind() == ErrorKind::NotFound { // Creates file.txt if not found
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error); // Panics if cannot create
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error); // Panics if cannot open
        }
    });

    let _username: Result<String, io::Error> = read_username_from_file(); // Reads username from file
    let _user: String = match _username {
        Ok(_username) => _username,
        Err(_) => String::from(""),
    };

    if _user.is_empty() { 
        let mut _input = String::new();
        std::io::stdin().read_line(&mut _input).expect("Problem reading input");
    }
    else {
        println!("{}", _user);
    }

    let _buf = BufReader::new(_file);
    let mut _count: usize = 0;
    let mut _vec: Vec<String> = Vec::new();
    for i in _buf.lines() {
        match i {
            Ok(i) => {
                _vec.push(i);
                _count = _count + 1;
            },
            Err(_) => continue,
        }
    }
    write_username_to_database(&_user);

    // Vector Index Handling
    // assert_eq!(_vec[_count], _user).expect("JOE MAMA");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let _file = File::open("file.txt");

    let mut _file = match _file {
        Ok(_file) => _file,
        Err(e) => return Err(e), 
    };
    
    let mut _username = String::new();
    _file.read_to_string(&mut _username)?;
    Ok(_username)

    // match _file.read_to_string(&mut _username) {
    //     Ok(_) => Ok(_username),
    //     Err(e) => Err(e),
    // }
}

fn write_username_to_database(_user: &String) {
    let mut _file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("database.txt")
        .unwrap_or_else(|error|{ // Opens file.txt
            if error.kind() == ErrorKind::NotFound { // Creates file.txt if not found
                File::create("database.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error); // Panics if cannot create
                })
            }
            else {
                panic!("Problem opening the file: {:?}", error); // Panics if cannot open
            }});

    if let Err(e) = writeln!(_file, "{}", _user) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
