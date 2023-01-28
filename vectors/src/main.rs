fn main() {
    let mut vector = vec![1,2,3];

    // Insert a new element in the vector
    vector.push(4);

    // Insert a new element in the vector depending on the position
    vector.insert(2,-1);

    // Remove the element from the vector
    vector.remove(vector.len() - 1);

    // Modify a value in the vector
    vector[0] = -10;

    // Interact with the vector
    let first_element = vector[0];
    let last_element = vector[vector.len() - 1];

    let last_element_diff = vector.pop().unwrap();

    println!("The Vector Factor is: {:?}", vector);

    println!("The First Element is: {}", first_element);
    println!("The Last Element is: {}", last_element);
    println!("The Vector Factor is: {}", last_element_diff);

}
