fn main() {
    // Arithmetic

    let mut a = 2+3*4; //  Follows presendence, operators
    println!("Arithmetric calculation: {}", a);

    // Increment / decrement values
    a = a + 1; // Default increment
    a += 1;
    a -= 1;

    // Modulo
    let rest_three = a % 3;
    println!("A % 3 = {}", rest_three);

    // Bitwise
    let c = 1 | 2; // | = OR, & = AND, ^ = XOR, ! = NOR
    println!("C is {}", c);
    // Shift >> / <<

    // Logical
    // ==, <, <=, >, >=
    // Returns boolean

}
