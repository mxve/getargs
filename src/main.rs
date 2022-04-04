use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    io::stdin().read_line(&mut String::new()).unwrap();
}
