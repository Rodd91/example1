extern crate ferris_says;
use ferris_says::say;
use std::io::{stdout, BufWriter};

// Function to print a string using Ferris says
fn ferris_print(message: &str) {
    let stdout = stdout(); 
    // Get handle to the standard output stream
    let mut writer = BufWriter::new(stdout.lock()); 
    // Wrap the standard output stream in a buffered writer
    let width = message.chars().count();
    // Count the number of characters in the input message
    say(message.as_bytes(), width, &mut writer).unwrap(); 
    // Call the say function with the message as a byte slice, the width of the message, and a mutable reference to the buffered writer. The unwrap method is used to handle any errors returned by say.
}


fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Transfer ownership

    // This line will cause a compile-time error due to ownership rules
    // println!("s1: {}", s1);

    let s3 = String::from("world");
    let s4 = &s3; // Borrowing , dereferencing the address

    // This line will work just fine
    println!("s4: {}", s4);

    let mut s5 = String::from("rust");
    let s6 = &mut s5; // Mutable borrowing

    // This line will cause a compile-time error due to mutable and immutable references coexisting
    // println!("s5: {}", s5);

    s6.push_str(" is safe!");
    println!("s6: {}", s6);



    
    ferris_print(s1);
}