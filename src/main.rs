use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("need at least 2 arguments")
    }
    let name = &args[1];
    let query = &args[2];

    println!("{}", name);
    println!("{}", query);
}
