fn main() {
    let mut arguments= std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let mut key = arguments.next().unwrap();
    print!( "The key is '{}' and the value is '{}'", key, value);
}
