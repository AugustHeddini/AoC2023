use std::{
    time::Instant,
    fs
};


fn main() {
    let start = Instant::now();

    let input = fs::read_to_string().unwrap();
    

    println!("Elapsed: {:?}", start.elapsed());
}
