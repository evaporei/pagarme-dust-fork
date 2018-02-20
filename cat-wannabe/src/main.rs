extern crate cat_wannabe;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("{}", cat_wannabe::concat(&args));
}
