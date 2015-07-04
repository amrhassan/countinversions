
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::env;
use std::process;

mod inversioncount;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} NUMBERS\n  Where NUMBERS is a path to a UTF-8 file containing unsigned integers separated by newlines", args[0]);
        process::exit(1);
    }

    let file_path = Path::new("IntegerArray.txt");
    let mut file = File::open(&file_path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    let numbers: Vec<u32> = file_content.lines().map(|x| x.trim().parse().unwrap()).collect();

    println!("Counting...");
    println!("In the given {} numbers I found {} inversions.",
        numbers.len(), inversioncount::count(numbers));
}
