fn main() {
    let mut arguments= std::env::args().skip(1);
    let key1 = arguments.next().unwrap();
    let value1 = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key1,value1);
    let mut key2 = arguments.next().unwrap();
    key2 = "dg_key".to_owned();

    print!( "The key is '{}' and the value is '{}'", key2, value1);
    
}
