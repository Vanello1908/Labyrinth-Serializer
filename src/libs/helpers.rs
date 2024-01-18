use std::fs;

pub fn get_bit(n: usize, k: usize) -> usize{
    (n >> k) & 1
}

pub fn check_file(filename: &String) -> Result<(), &'static str>{
    if fs::metadata(filename).is_err(){
        return Err("Invalid filename!");
    }  
    Ok(())
}