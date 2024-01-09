fn main() {
    
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