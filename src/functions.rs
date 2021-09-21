pub fn run() {
    greeting("Hello", "Jane");

    let get_sum = add(5, 5);
    println!("sum is {}", get_sum);

    let n3: i32 = 4;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum is {}", add_nums(3, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet yout", greet, name);
}
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
