use std::fs;
use std::io::stdin;
fn main() {
    let contents = fs::read_to_string("./file.txt")
        .expect("Something went wrong reading the file");

    println!("Enter a word : ");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    println!("[{}]", &input);

    let rs = contents.matches(&input.trim()).count();

    println!("The result : {} ", rs);

}