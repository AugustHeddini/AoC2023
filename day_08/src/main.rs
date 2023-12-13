use std:: {
    time::Instant,
    fs,
    collections::BTreeMap
};

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input").unwrap();

    


    println!("Elapsed: {:?}", start.elapsed());
}
