pub fn run() {
    let name = "brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 30;
    println!("My name is {} and I am {}", name, age);

    //DEFINBE CONSTANT
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //ASIGN MULTIPLE VARS
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
