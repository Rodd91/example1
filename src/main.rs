fn main() {
    //-------------------------------------------------------------//OWNERSHIP

    /* 
    let s1 = String::from("hello");
    let s2 = s1; // Transfer ownership

    // This line will cause a compile-time error due to ownership rules
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    */

    //-----------------------------------------------------------// Borrowing

    /* 
    let s3 = String::from("world");
     let s4 = &s3; // Borrowing

     // This line will work just fine
    println!("s3: {}", s3);
    println!("s4: {}", s4);

    */

    //-----------------------------------------------------------// Mutable borrowing

    /* 
    let mut s5 = String::from("rust");
    let s6 = &mut s5; // Mutable borrowing

    // This line will cause a compile-time error due to mutable and immutable references coexisting
    println!("s5: {}", s5);

    s6.push_str(" is safe!");
    println!("s6: {}", s6);
    println!("s5: {}", s5);
    */
    
}