pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    numbers[2] = 30;

    numbers.push(5);
    numbers.push(6);

    numbers.pop();

    println!("{:?}", numbers);
    println!("single value : {}", numbers[0]);
    println!("single value : {}", numbers[2]);
    println!("Array Length : {}", numbers.len());
    println!("Array oocupies : {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers;
    println!("Slice : {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number Vec: {:?}", numbers);
}
