use std::mem;

fn main() {

    // A const in rust will be replaced on compile time
    // The value will have no predictable address in memory
    const MEANING_OF_LIFE: u8 = 42;

    println!("The meaning of life is {}", MEANING_OF_LIFE);
    
    // Global static value
    // If this is mutable, it is unsafe to use do to being writeable by multiple routines
    static mut Z :i32 = 123; 

    unsafe {
        // I promise I will be very careful within this code
        println!("z is currently {}", Z);
    }

    stack_and_heap(); // Call the function stack and heap

}

/*
 *  Stack and Heap
 */

 /*
  * Stack
  *     At the bottom of the memory map
  *     LIFO (Last-in-first-out) --> Can be cleaned up after using
  *     Variables declared with let are avaiable in scope and will be deleted
  *     Fast
  *     Limited memory
  */

  /*
   * Heap
   *    Long-Term-Storage option
   *    Assigned by using for example with the generic box type
   *        // Represents a pointer to the heap instead of the stack
   *        let x = Box::new(5);
   *    All heap allocated objects have to be the deref operator *
   *    For example:
   *        println("x = {}", *x);
   */

struct Point {
    x: f64,
    y: f64
}

fn original_point() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {
    let p1 = original_point();
    let p2 = Box::new(original_point());

    println!("p1 takes {} bytes", mem::size_of_val(&p1));
    println!("p2 takes {} bytes", mem::size_of_val(&p2));

    let p3 = *p2; // Follows the boxed value and assigns it to p3
    println!("x: {}, y: {}", p3.x, p3.y);
}