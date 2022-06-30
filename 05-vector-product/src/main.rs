use std::io;

struct Vector{
    x: i64,
    y: i64,
    z: i64
}

fn dot(_vec1: &Vector, _vec2: &Vector) {
    let product = (_vec1.x * _vec2.x) + (_vec1.y * _vec2.y) + (_vec1.z * _vec2.z);
    println!("\nDot Product = {}", product);
}

fn cross(_vec1: &Vector, _vec2: &Vector) {
    let cross = Vector {
        x: ((_vec1.y * _vec2.z) - (_vec1.z * _vec2.y)),
        y: ((_vec1.x * _vec2.z) - (_vec1.z * _vec2.x)),
        z: ((_vec1.x * _vec2.y) - (_vec1.y * _vec2.x))
    };
    println!("\nCross Product = <{}, {}, {}>", cross.x, cross.y, cross.z);
}

fn main() {
    let mut _vec1 = Vector {
        x: 0,
        y: 0,
        z: 0
    };
    let mut _vec2 = Vector {
        x: 0,
        y: 0,
        z: 0
    };
    let mut _input = String::new();

    println!("\n1st Vector Input: 'a b c' (<a, b, c>)");
    std::io::stdin().read_line(&mut _input).expect("Could not read input");
    let mut _nums: Vec<i64> = _input.trim().split(' ').flat_map(str::parse::<i64>).collect();

    _vec1.x = _nums[0];
    _vec1.y = _nums[1];
    _vec1.z = _nums[2];

    _input.clear();
    println!("\n2nd Vector Input: 'd e f' (<d, e, f>)");
    std::io::stdin().read_line(&mut _input).expect("Could not read input");
    let mut _nums: Vec<i64> = _input.trim().split(' ').flat_map(str::parse::<i64>).collect();

    _vec2.x = _nums[0];
    _vec2.y = _nums[1];
    _vec2.z = _nums[2];

    dot(&_vec1, &_vec2);
    cross(&_vec1, &_vec2);
}
