fn main() {

    // Arrays
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    let values = [1; 10];
    println!("{:?}", values);

    let first_list = numbers[2];
    let last_list = numbers[numbers.len() - 1];

    println!("{:?}", first_list);
    println!("{:?}", last_list);

    // Tuples
    
}
