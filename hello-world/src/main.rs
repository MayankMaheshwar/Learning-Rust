use std::io;

fn main() {
    println!("Hello, world!");
    println!("guess your age");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Error reading");

    println!("guess your age: {}", guess);


}
