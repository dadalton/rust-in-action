// ########## CHAPTER 3 ###############
// ###### COMPOUND DATA TYPES ########
/*
- Composing data with structs
- Creating enumerated data types
- Adding methods and handling errors in a type-safe manner
- Defining and implementing common behavior with traits
- Understanding how to keep implementation details private
- Using cargo to build documentation for your project 

Two key building blocks: struct and enum
*/
/*
#![allow(unused_variables)] // relax compiler warnings

    type File = String;     // type alias

    fn open(f: &mut File) -> bool {
        true
    }

    fn close(f: &mut File) -> bool {
        true
    }

#[allow(dead_code)]         // relax compiler warning about unused functions
fn read(f: &mut File,
        save_to: &mut Vec<u8>) -> ! { // ! type indicates the function never returns
    unimplemented!()        // macro that crashes if encountered 
}

fn main() {
    let mut f1 = File::from("f1.txt"); // File inherits String's methods through type alias
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
*/
// UNIT TYPE () - formally a zero-length tuple
// - expresses a function that returns no value
// - eg functions without a return type return (), as do expressions with a ;

// "NEVER" TYPE ! - indicates a function never returns

// 3.2 Modeling files with struct

// struct allows creation of a composite type 
// - analagous to object or record in other languages
/*
#[derive(Debug)]        // allows println! to print File
    struct File {
        name: String,   // fields require explicit lifetimes if they're a ref to another object
        data: Vec<u8>,  // here the field lifetimes are implicit
    }

    fn main() {
        let f1 = File {
            name: String::from("f1.txt"),  // generate owned strings from string literals
            data: Vec::new(),              // simulate empty file
        };

        let f1_name = &f1.name;            // accessing fields by reference prevents use after move
        let f1_length = &f1.data.len();

        println!("{:?}", f1);
        println!("{} is {} bytes long", f1_name, f1_length);
    }
*/
// NEWTYPE PATTERN
/*
struct Hostname(String);

fn connect(host: Hostname) {
    println!("connected to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("localhost");
    let host = Hostname ( ordinary_string.clone() );

    connect(ordinary_string);
}
*/

// ------ 3.4 RETURNING ERRORS ------
/*
Dealing with hardware is unreliable. There may be hardware faults, OS permission issues, etc.

A simple method to signal an error is to check the value of a global variable.

*/
// A Rust version of global variable error check:
/*
static mut ERROR: i32 = 0; // global variable with a static lifetime valid for life of the program

// ...

fn main() {
    let mut f = File::new("something.txt");

    read(f, buffer);
    unsafe {            // modifying static mut variables requires unsafe
        if ERROR != 0 {
            panic!("An error has occurred while reading the file")
        }
    }

    close(f);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred while closing the file")
        }
    }
}
*/
/*
use rand::{random};

    static mut ERROR: isize = 0;

    struct File;

    #[allow(unused_variables)]
    fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
        if random() && random() && random() { // random() is a coin flip, this will be true 1/8 times
            unsafe {
                ERROR = 1;
            }
        }
        0
    }

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!")
        }
    }
}

*/

/* CONST VS LET
- let allows interior mutability
- at the compiler level, let allows aliasing - referencing - the same data 
in multiple locations simultaneously
- mutable borrows never alias data
*/