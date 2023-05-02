

//Function to print a string using Ferris_says
fn ferris_print(message: &str) {
    // Define a function named "ferris_print" that takes a string slice as input
    let stdout = stdout();
    // Call the "stdout" function from the "std::io" module to get a handle to the standard output stream
    let mut writer = stdout.lock();
    // Get a mutable reference to the standard output stream
    let width = message.chars().count();
    // Count the number of characters in the input message using the "chars" method and store the result in the "width" variable
    say(message.as_bytes(), width, &mut writer).unwrap();
    // Call the "say" function with the message as a byte slice, the width of the message, and a mutable reference to the standard output stream. The "unwrap" method is used to handle any errors returned by "say".
}




fn main() {
    //-------------------------------------------------------------//OWNERSHIP
    let s1 = String::from("hello");
    let s2 = s1; // Transfer ownership

    // This line will cause a compile-time error due to ownership rules
    //say(s1.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();

    say(s2.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();
    
    
    //-----------------------------------------------------------// Borrowing 
    let s3 = String::from("world");
     let s4 = &s3; // Borrowing , dereferencing the address

    say(s3.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();
    say(s4.as_bytes(), 15, &mut BufWriter::new(stdout().lock())).unwrap();




    //-----------------------------------------------------------// Mutable borrowing
    let mut s5 = String::from("rust");
    let s6 = &mut s5; // Mutable borrowing

    // This line will cause a compile-time error due to mutable and immutable references coexisting
    println!("s5: {}", s5);

    s6.push_str(" is safe!");
    println!("s6: {}", s6);
    

    ferris_print(s6);
    
}