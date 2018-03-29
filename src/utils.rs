use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IndexError;
impl fmt::Display for IndexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "index error")
    }
}

impl Error for IndexError {
    fn description(&self) -> &str {
        "invalid indicies were used"
    }
}

pub fn get_two<T>(data: &mut Vec<T>, i: usize, j: usize) -> Result<(&mut T, &mut T), IndexError> {
    if i == j  || i >= data.len() || j >= data.len() {
        return Err(IndexError)
    }
    assert!(i != j);
    assert!(i < data.len());
    assert!(j < data.len());
    unsafe {
        let a = &mut *(data.get_unchecked_mut(i) as *mut T);
        let b = &mut *(data.get_unchecked_mut(j) as *mut T);
        Ok((a, b))
    }
}

pub fn get_all<T>(data: &mut Vec<T>) -> Vec<&mut T> {
    let mut return_vec = Vec::new();
    unsafe {
        for i in 0..data.len() {
            return_vec.push(&mut *(data.get_unchecked_mut(i) as *mut T)); 
        }
    }
    return_vec
}
