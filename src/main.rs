// // extern crate ferris_says;           //import external crate that provides function to print a string in a bubble with Ferris Crab ASCII art
// // use ferris_says::say;               //import function which takes a byte slice, a width, and a mutable reference to a writer, writes message to the writer
// // use std::io::{stdout, BufWriter};   //module that allows a rust program to write to console efficently by using a buffer writer

// fn main() {
//     // //-------------------------------------------------------------//OWNERSHIP
//     // let s1 = String::from("hello");
//     // let s2 = s1; // Transfer ownership

//     // // This line will cause a compile-time error due to ownership rules
//     // //say(s1.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();

//     // say(s2.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();
    
    
//     // //-----------------------------------------------------------// Borrowing 
//     //  let s3 = String::from("world");
//     //  let s4 = &s3; // Borrowing , dereferencing the address

//     // say(s3.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();
//     // say(s4.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();




//     //-----------------------------------------------------------// Mutable borrowing
//     // let mut s5 = String::from("rust");
//     // let s6 = &mut s5; // Mutable borrowing

//     // // This line will cause a compile-time error due to mutable and immutable references coexisting
//     // println!("s5: {}", s5);

//     // s6.push_str(" is safe!");
//     // println!("s6: {}", s6);
    

//     // ferris_print(s6);
    
// }

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Transfer ownership

    // This line will cause a compile-time error due to ownership rules
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3 = String::from("world");
    let s4 = &s3; // Borrowing

    // This line will work just fine
    println!("s4: {}", s4);

    let mut s5 = String::from("rust");
    let s6: &mut String = &mut s5; // Mutable borrowing

    // This line will cause a compile-time error due to mutable and immutable references coexisting
    //println!("s5: {}", s5);// immutable borrow

    s6.push_str(" is safe!"); //mutable borrow later used here
    println!("s6: {}", s6);
    //now it will print because it has been changed
    println!("s5: {}", s5);
}
