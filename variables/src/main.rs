fn main() {

    // let <name variable> = <value of variable>
    // let <name variable>: <type of variable> = <value of variable>
    // let mut <name variable> = <value of variable>

    let mut number_one = 10;
    let number_two: i32 = 15;

    // const <NAME VARIABLE> : <type of variable> = <value of variable>

    const VALUE: i32 = 10;

    number_one = 100;

    let result = number_one + number_two + VALUE;

    println!("The value of the ({} + {} + {}) is: {}", number_one, number_two,
            VALUE, result);

    // Shadowing

    let value_variable: i32 = 10;

    println!("The value of shadowing is: {} ", value_variable);

    let value_variable = 20;

    println!("The value of shadowing 2 is: {} ", value_variable);
}
