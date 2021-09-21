pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    numbers[2] = 30;

    println!("{:?}", numbers);
    println!("single value : {}", numbers[0]);
    println!("single value : {}", numbers[2]);
    println!("Array Length : {}", numbers.len());
    println!("Array oocupies : {} bytes", std::mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers;
    println!("Slice : {:?}", slice);
}
