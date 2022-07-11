use rand::Rng;
use binary_search::NumList;

fn main() {
    let mut _vec_input = vec![1, 5, 3, 2, 4];
    let mut _nums = NumList{
        _nums_vec: _vec_input,
    };
    _nums._nums_vec.sort_by(|a: &i32, b: &i32| a.partial_cmp(b).unwrap());
    let _target: i32 = rand::thread_rng().gen_range(1..6);

    let (_found, _index) = NumList::_binary_search(&_nums, _target);
    if _found == true {
        println!("FOUND! Value: {} was found at {}", _nums._nums_vec.get(_index).unwrap(), _index);
    }
    else {
        println!("{} is not in the _nums vector :(", _target);
    }
}
