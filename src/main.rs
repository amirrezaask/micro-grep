use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("need at least 2 arguments")
    }
    let name = &args[1];
    let query = &args[2];

    let content = fs::read_to_string(name).expect("cannot read file");

    println!("{}", content);
}
