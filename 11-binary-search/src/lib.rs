pub struct NumList {
    pub _nums_vec: Vec<i32>,
}

impl NumList {
    pub fn _binary_search (&self, _target: i32) -> (bool, usize) {
        let mut _left = 0;
        let mut _right = self._nums_vec.len()-1;
        while _left <= _right {
            let _mid: usize = _left + (_right-_left) / 2;
            if self._nums_vec[_mid] == _target {
                return (true, _mid);
            }
            else if self._nums_vec[_mid] < _target {
                _left = _mid + 1;
            }
            else {
                _right = _mid - 1;
            }
        }
        return (false, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_check() {
        let _check_numlist = NumList {
            _nums_vec: vec![1, 2, 3],
        };
        let (_found, _index) = _check_numlist._binary_search(2);
        assert!(_found);
    }
    
    #[test]
    fn spooked(){
        panic!("OMG so scary! :(");
    }
}