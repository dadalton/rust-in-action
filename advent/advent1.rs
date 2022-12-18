//Advent of code #1 - elf calorie counting
 
 
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;



fn main() {
    let mut f = File::open("caloriesample.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    let re = Regex::new(r"(\n)").unwrap();
    let count: i32 = 0;

    while ()

    // read a line into buffer
    reader.read_line(&mut buffer)?;
    if (reader != "\n") { // line has content
      // add calorie
      count = 0;
    }

    elseif (reader == "\n" && //no content) {
      count += 1;
    }

    if (count == 2) {
      // next elf
    }

    println!("{buffer}");
    Ok(())
}

// newline counter - if newline == 2, store in array as total calorie


 for (let i = 0; i < 10; i++) {
    // code to execute 10 times
  }




 
 #[derive(Debug)]
 pub struct File {
   name: String,
   data: Vec<u8>,
 }
 
 impl File {
   /// New files are assumed to be empty, but a name is required.
   pub fn new(name: &str) -> File {
     File {
       name: String::from(name),
       data: Vec::new(),
     }
   }
 
   /// Returns the file's length in bytes.
   pub fn len(&self) -> usize {
     self.data.len()
   }
 
   /// Returns the file's name.
   pub fn name(&self) -> String {
     self.name.clone()
   }
 }
 
// // Objects
// struct MyStruct {
//   // properties
//   name: String,
//   age: i32,
// }

// // methods
// impl MyStruct {
//   fn print_info(&self) {
//       println!("Name: {}, Age: {}", self.name, self.age);
//   }
// }

// // creating an object
// let obj = MyStruct {
//   name: "John Doe".to_string(),
//   age: 30,
// };

//  fn main() {
//    let f1 = File::new("f1.txt");
 
//    let f1_name = f1.name();
//    let f1_length = f1.len();
 
//    println!("{:?}", f1);
//    println!("{} is {} bytes long", f1_name, f1_length);
//  }


 for (let i = 0; i < 10; i++) {
  // code to execute 10 times
}

while (condition) {
  // code to execute until condition is false
}

 // Declare a struct called "Person" that has two fields: "name" and "age"
struct Person {
  name: String,
  age: u8,
}

// Define a function called "greet" that takes a "Person" instance as an argument
// and returns a string
fn greet(person: Person) -> String {
  // Use the "format!" macro to create a string containing the person's name and age
  format!("Hello, my name is {} and I am {} years old.", person.name, person.age)
}

fn main() {
  // Create a new instance of the "Person" struct, using field shorthand syntax
  let person = Person { name: "John Doe".to_string(), age: 30 };

  // Call the "greet" function, passing the "person" instance as an argument,
  // and store the result in a variable called "message"
  let message = greet(person);

  // Use the "println!" macro to print the "message" to the console
  println!("{}", message);
}