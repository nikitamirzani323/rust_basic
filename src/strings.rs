pub fn run() {
    let mut hello = String::from("Hello");

    //GET LENGTH
    println!("Length:{}", hello.len());

    //PUSH CAR
    hello.push('W');
    //PUSH STRING
    hello.push_str("orld!");
    //CAPACITY IN BYTES
    println!("Capacity: {}", hello.capacity());
    //CHECK IF EMPTY
    println!("Is Empty: {}", hello.is_empty());
    //CONTAINS
    println!("Contains 'World' {}", hello.contains("world"));
    //REPLACE
    println!("Replace: {}", hello.replace("World", " sange"));
    //LOOP THROUGH STRING BY WHITESPACE
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //ASSERTION TESTING
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello)
}
