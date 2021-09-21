pub fn run() {
    let age = 18;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Bartender: What would you like to drink");
    } else if age < 21 && check_id {
        println!("Tai else if");
    } else {
        println!("Tai Default");
    }

    let is_of_age = if age >= 21 { true } else { false };
    println!("is of age : {}", is_of_age);
}
