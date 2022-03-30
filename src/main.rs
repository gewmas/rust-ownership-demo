// https://www.tutorialspoint.com/rust/rust_borrowing.htm
//
// What is Ownership?
//
// Each value in Rust has a variable that is called owner of the value.
//     Each data can have only one owner at a time.
//     Two variables cannot point to the same memory location. The variables will always be pointing to different memory locations.
//
// Transferring Ownership
//
// The ownership of value can be transferred by −
//     Assigning value of one variable to another variable.
//     Passing value to a function.
//     Returning value from a function.

// Example 1
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy
// fn main() {
//     let x = 5;
//
//     // {
//     //     let y = x;
//     //     println!("x = {}, y = {}", x, y);
//     // }
//
//     let z = x;
//     println!("x = {}, z = {}", x, z);
// }


// Example 2.1
// fn main() {
//     let s1 = String::from("hello");
//
//     // {
//     //     let s2 = s1;
//     //     println!("{}, world!", s2);
//     // }
//
//     let s3 = s1;
//     println!("{}, world!", s1);
}


// Example 2.2
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-clone
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//
//     println!("s1 = {}, s2 = {}", s1, s2);
// }


// Example 3 - Ownership and Functions
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions
// fn main() {
//     let s = String::from("hello");  // s comes into scope
//
//     takes_ownership(s);             // s's value moves into the function...
//     // ... and so is no longer valid here
//     // println!("{}", s);
//
//     let x = 5;                      // x comes into scope
//
//     makes_copy(x);                  // x would move into the function,
//     // but i32 is Copy, so it's okay to still
//     // use x afterward
//     // println!("{}", x);
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
// // special happens.
//
// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
// // memory is freed.
//
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// Example 4 - Return Values and Scope
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope
// fn main() {
//     let s1 = gives_ownership();         // gives_ownership moves its return
//     // value into s1
//
//     let s2 = String::from("hello");     // s2 comes into scope
//
//     let s3 = takes_and_gives_back(s2);  // s2 is moved into
//     // takes_and_gives_back, which also
//     // moves its return value into s3
//     // println!("{}", s2);
//
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// // happens. s1 goes out of scope and is dropped.
//
// fn gives_ownership() -> String {             // gives_ownership will move its
//     // return value into the function
//     // that calls it
//
//     let some_string = String::from("yours"); // some_string comes into scope
//
//     some_string                              // some_string is returned and
//     // moves out to the calling
//     // function
// }
//
// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//     // scope
//
//     a_string  // a_string is returned and moves out to the calling function
// }


// Example 5 - References and Borrowing
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing
// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.


// Example 6 - Mutable References
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &mut s;
//     println!("{}", r1);
//
//     let r2 = &mut s;
//     println!("{}", r2);
//
//     // println!("{}, {}", r1, r2);
// }


// Example 7 - Immutable and Mutable References
// fn main() {
//     let mut s = String::from("hello");
//
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s;
//     println!("{}, {}, and {}", r1, r2, r3);
// }


// Example 8 - Dangle Reference
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references
// fn main() {
//     let reference_to_nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
