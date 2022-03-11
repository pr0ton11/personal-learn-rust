#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    
    let a: u8 = 123; // Unsigned int, 8 bits, 0 - 255, immutable
    let b: i8 = -123; // Signed int, 8 bits, -128 - 127, immutable

    println!("a = {}, b = {}", a, b);

    let mut c: i8 = 0; // Signed int, 8 bits, -128 - 127, mutable
    // Intrestingly, the compiler even detects that this variable was not read before updating it
    // Which could be optimized when initialized just before first reading
    c = 127; // Update value from 0 to 127

    println!("C has been updated from 0 to {}", c);

    // Implicit typing
    let d = 123456789; // i32 is autodetected
    // Using memory package to check the size that &d (pointer to d) actually takes
    println!("Value of d: {}, takes {} bytes", d, mem::size_of_val(&d));

    // usize & isize
    // native length of processor type (e.g 64 bits)
    // usize : unsigned 64 bit = u64
    // isize : signed 64 bit = i64

    let e: isize = 123;
    let size_of_e = mem::size_of_val(&e);
    println!("Value of e: {}, takes {} bytes, {}-bit OS", e, size_of_e, size_of_e*8);

    // Chars
    let f : char = 'x'; // Char that is UTF-32 compatible
    println!("Character {}, takes {} bytes", f, mem::size_of_val(&f));

    // Floats
    // f32, f64 -> IEEE754
    // Implicit typing = f64
    let g: f32 = 2.5;
    println!("Float {}, takes {} bytes", g, mem::size_of_val(&g));

    // Boolean
    let h : bool = false; // true
    println!("Boolean {}, takes {} bytes", h, mem::size_of_val(&h));

}
