fn main() {
    // str -> inmutable array -> store Stack
    // String -> mutable array -> store Heap

    let variable_str = "Hi, I'm a str type";
    let mut variable_string = String::from("Hello, I'm a string type");

    // Add a character to the string
    variable_string.push(' ');

    // Add a new String to the string
    variable_string.push_str("use the push option");

    // Convert the str to a string
    let new_str = variable_str.to_string();

    // Use of the operatos == and !=

    let same_string = new_str == "Hi, I'm a str type".to_string();

    let different_string = new_str != "Hi, I'm a string type".to_string();

    println!("str is: -> {}", variable_str);
    println!("String is: -> {}", variable_string);
    println!("New string is: -> {}", new_str);
    println!("Operator == is: -> {}", same_string);
    println!("Different is : {}", different_string);
}
