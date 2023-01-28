fn main() {
    println!("Hello, world!");
    salute_user();
    let result = add_numbers(2,2);
    let factorial = factorial(3);
    println!("The result is: {}", result);
    println!("The factorial is: {}", factorial);
}

fn salute_user() {
    println!("Hello, User!");
}

// Recursive function
fn factorial(number: u32) -> u32 {
    if number == 1 {
        return number;
    }
    factorial(number - 1) * number
}

fn add_numbers(number_one: i32, number_two: i32) -> i32 {
    number_one + number_two
}
