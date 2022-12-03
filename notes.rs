
// Multiple ways to declare integers
/*
fn main() {
    let a = 10; // Compiler can infer types
    let b: i32 = 20;
    let c = 30i32; // numeric types can include type annotation in literal
    let d = 30_i32; // Compiler is agnostic to underscore - used for legibility
    let e = add(add(a,b), add(c,d));

    println!("( a + b) + ( c + d ) = {}", e);
}

fn add(i: i32, j: i32) -> i32 { // type declarations required when defining functions
    i + j // functions return last expression's result - return not required
}
*/

/*
fn main() {
    let three = 0b11; // base 2 - binary
    let thirty = 0o36; // base 8 - octal
    let three_hundred = 0x12c; // base 16 - hexadecimal

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
*/

/* types for scalar numbers
i8, i16, i32, i64 - signed integers from 8 bit to 64 bit
u8, u16, u32, u64 - unsigned integers from 8 bit to 64 bit
f32, f64 - floating point numbers in 32 and 64 bit
    - also used to represent inf, -inf, NaN 
isize, usize - signed/unsigned integers that assume the CPU's native width
*/


/*
fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) { // as operator casts an operand to a different type
        println!("Ten is less than one hundred.")
    }
}
*/

/*
use std::convert::TryInto;

    fn main() {
        let a: i32 = 10;
        let b: u16 = 100;

        let b_ = b.try_into() // returns a Result type, which contains either a success value or an error value
                  .unwrap();  // handles the success value - error value crashes the program

        if a < b_ {
            println!("Ten is less than one hundred");
        }
    }
*/

/*
fn main() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2); // f32 test passes
    assert!(xyz.0 + xyz.1 == xyz.2); // f64 test fails
}
*/

/*
fn main() {
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON); //the tolerance f32::EPSILON delegates the comparison to the CPU
}
*/

// NaN values are never equal - almost all operations interacting with NaN return NaN
/* This program always crashes
fn main() {
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x);
}
*/

/*//   |     unresolved import
//     |     help: a similar path exists: `std::num`
use std::num::Complex;

    fn main() {
        let a = Complex { re: 2.1, im: -1.2 }; // literal syntax
        let b = Complex::new(11.1, 22.2);      // new() static method
        let result = a + b;

        println!("{} + {}i", result.re, result.im)
    }
*/

/* -------------- FOR LOOP -----------------
for item in container {
    stuff
}

---- modify each item during the loop:
for item in &mut collection {
    stuff
}

---- read-only
for item in &collection {
    stuff
}

---- use _ when local variable is not used within a block
---- exclusive range (n..m)
---- inclusive range (n..=m)
for _ in 0..10 {
    stuff
}

*/

/* ---- LOOP - continues until a break statement or program is terminated externally

loop {
    stuff
}

---- LOOP LABEL
'outer: for x in 0.. {
    for y in 0.. {
        for z in 0.. {
            if x + y +z > 1000 {
                break 'outer; // ' specifies loop to break out of
            }
            
            stuff
        }
    }
}
*/

/*  ------- EXPRESSIONS - STATEMENTS -------
Expressions return values, and almost everything is an expression
Statements are not expressions and appear in three places
 1. Expressions delimited by ; 
 2. Binding a name to a value with =  
 3. Type declarations
- 1 is an expression statement, 2 & 3 are declaration statements
*/

/*
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let n = 123456; // Statement
    let description = if is_even(n) {
        "even"      // Expression
    } else {
        "odd"       // Expression
    };
    println!("{} is {}", n, description);
}
*/

/* ---------- MATCH - TYPE AWARE PATTERN MATCHING
Match warns you if you haven't considered a relevant alternative
Analogous to switch in other languages
*/

/*
match item {
    0           => {}, // match a single value

    10 ..= 20   => {}, // match any value within range

    40  |  80   => {}, // matches when either side matches

    _           => {}, // match everything
}

*/

// ---------- FUNCTION SYNTAX --------
/*
fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn      // KEYWORD
add     // IDENTIFIER
(i:     // IDENTIFIER
i32,    // TYPE
j: i32) // PARAMETER
->      // indicates return
i32     // RETURN TYPE
{       // begin code block
}
*/

// ---------- REFERENCES --------------
/*
fn main() {
    let a = 42;
    let r = &a;     // reference operator &
    let b = a + *r; // dereference operator *

    println!("a + a = {}", b);
}
*/

// ---------- LIFETIMES ---------------
/*
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
*/
// <'a, 'b> declares lifetime variables 'a, 'b, within scope of the the function
// i: &'a i32 binds 'a to the lifetime of i - reads "parameter i is a reference to an i32 with lifetime a"
// the compiler can infer most lifestimes without explicit annotation
// difficult cases - such as functions that accept multiple refs as arguments or return a reference - may require explicit annotation

/*


STRING STUFF HEAH


*/

// ---------- ARRAYS, SLICES, VECTORS ------------
/* 
- Arrays [T; n] are fixed-width and extremely lightweight
- Vectors are growable at the cost of a small runtime penalty

An ARRAY is a tightly-packed collection of the same thing
- its items are replacable but its size is fixed

[1, 2, 3]
[0; 100] // 0 is repeated 100 times
*/

fn main() {
    let one             = [1, 2, 3];
    let two: [u8; 3]    = [1, 2, 3]; // array [T; n] - T is the type, n # elements
    // [u8; 3] is a different type to [u8; 4] - size matters
    let blank1          = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n+10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}
/* OUTPUT
[1, 2, 3]:      1 + 10 = 11     2 + 10 = 12     3 + 10 = 13     ([1, 2, 3] = 6)
[1, 2, 3]:      1 + 10 = 11     2 + 10 = 12     3 + 10 = 13     ([1, 2, 3] = 6)
[0, 0, 0]:      0 + 10 = 10     0 + 10 = 10     0 + 10 = 10     ([0, 0, 0] = 0)
[0, 0, 0]:      0 + 10 = 10     0 + 10 = 10     0 + 10 = 10     ([0, 0, 0] = 0)
*/
/*
- Typically arrays interact with another type called a slice - [T]
- The slice itself is interacted with by reference &[T] - also called a slice
- Requesting an out of bounds elements crashes (panics) the program

SLICES [T] are dynamically sized array-like objects
- this means their size is unknown at compile time
- however, like arrays, they don't expand or contract
- It's easier to implement traits for slices than arrays
- Slices can act as a view on arrays - and other slices
- "view" meaning fast, read-only access
- slice size is made up of two usize components - a pointer and a length

In practice there's not a significant difference between arrays and slices
- the implementation details are important for performance-critical applications

VECTORS - Vec[T]

see ch2-introducing-vec.rs

------------- THIRD-PARTY CODE ---------------

