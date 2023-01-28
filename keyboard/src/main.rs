use std::io;

fn main() {
    println!("Insert the name of the user");
    let mut username = String::new();

    // Return a result(Sucess/Error).
    io::stdin().read_line(&mut username); // the username use a reference with the permission of mutable.

    let username = username.trim(); // remove the salt of line.

    println!("Insert the age of the user");

    let mut age = String::new();

    io::stdin().read_line(&mut age);

    let age = age.trim();

    // Convert a string in integer format.
    let age: i32 = age.parse().unwrap();

    println!("The user is : {} and the age is : {}", username, age);


}
