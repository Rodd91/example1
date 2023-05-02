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
    //-------------------------------------------------------------//OWNERSHIP

    let s1 = String::from("hello");
    let s2 = s1; // Transfer ownership, ownership is moved
    
    // This line will cause a compile-time error due to ownership rules
    //println!("s1: {}", s1);
    
    println!("s2: {}", s2);
    ferris_print(&s2);

    ////ownership rules:
    ////Each value in Rust has a single owner, which is a variable binding.
    ////When the owner goes out of scope, the value will be deallocated automatically.
    ////There are three ways to interact with data: ownership, borrowing, and mutable borrowing
    

    //-----------------------------------------------------------// Borrowing

    
    let s3 = String::from("world");
    let s4 = &s3; // Borrowing a reference w/ &

    // This line will work just fine because ownership has not been lost by s3
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    //using ferris to print the string
    ferris_print( &s3);
    ferris_print( &s4);

    ////Borrowing Rules:
    ////Immutable Borrowing: You can borrow a reference to a value without transferring ownership by using the & operator. 
    ////This creates an immutable reference, meaning that the borrowed value cannot be modified through the reference.

    //-----------------------------------------------------------// Mutable borrowing

    
    let mut s5 = String::from("rust");
    let s6 = &mut s5; // Mutable borrowing, modify value through a reference

    // This line will cause a compile-time error due to mutable and immutable references coexisting
    //println!("s5: {}", s5); //immutable borrowing, so it will cause an error

    s6.push_str(" is safe!"); //mutable borrowing, so it will work
    println!("s6: {}", s6);
    ferris_print(s6);

    //this line will now work since it is after the mutable borrowing lines 
    println!("s5: {}", s5);
    ////Mutable Borrowing: If you want to modify a value through a reference, you can use mutable borrowing with the &mut operator.
    ////Restrictions:
    ////You cannot have both mutable and immutable references to a value in the same scope.
    ////You can have multiple immutable references, but you cannot have more than one mutable reference to a value in the same scope.
    
}