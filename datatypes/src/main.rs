fn main() {

    /* DataTypes
        i8, i16, i32, i64, i128 --> with sign - +
        u8, u16, u32, u64, u128 --> without sign
    */

    let number_one: i8 = -10;
    let number_two: u8 = 10;

    /* Char --> UTF-8 */

    let character = 'a';

    /* Float
        f32 or f64
    */

    let real: f32 = 12.5;

    /* Boolean */

    let result: bool = true;

    println!("the value is {} and {}", number_one, number_two);
    println!("the value of character is {}", character);
    println!("the value of float is {}  ", real);
    println!("the value of boolean is {}  ", result);
}
