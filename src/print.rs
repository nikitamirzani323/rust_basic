pub fn run() {
    //BASIC CONSOLE
    println!("Hello from to the print.rs");

    //BASIC FORMATING
    println!("Number: {}", 1);
    println!("{} is from: {}", "Brad", "Mass");

    //POSITIONAL ARGUMENTS
    println!(
        "{0} is from {1} and {2} likes tos {3}",
        "Brad", "Mass", "code", "rust"
    );
    //NAMED ARGUMENTS
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    //PLACEHOLDER TRAITS
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //PLACEHOLDER FOR DEBUG TRAITS
    println!("{:?}", (12, true, "hello"));
}
