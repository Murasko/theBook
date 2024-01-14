fn main() {
    println!("Hello, world!");

    let some_string = String::from("hello");
    
    // This would work because some_string is still in scope and not yet moved/dropped
    //let new_string = some_string;

    takes_ownership(some_string);
        
    let new_string = some_string; // This will not work because some_string has been moved to the takes_ownership function and got dropped after the end of the function

    let x = 5;

    makes_copy(x);

    println!("{}", x); // This will work because x is a primitive type and it's value is copied to the makes_copy function
}

// A scope is the range within a program for which an item is valid
fn variable_scope() {
    // s is not valid here, it's not yet declared
    let s = "hello"; // s is valid from this point forward
    // do stuff with s
} // this scope is now over, and s is no longer valid


fn string_type() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer); // This will print 5
} // Here, some_integer goes out of scope. Nothing special happens

fn takes_ownership(another_string: String) { // another_string comes into scope
    println!("{}", another_string); // This will print `hello`
} // Here, another_string goes out of scope and `drop` is called. The backing memory is freed