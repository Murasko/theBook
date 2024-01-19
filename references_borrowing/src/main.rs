// Refrencing is done by using the '&' symbol
// Dereferencing is done by using the '*' symbol (will get covered later)

// With references, also known as borrowing a value, we can use it without taking ownership of it

// References are immutable by default

// We can only have one mutable reference to a particular piece of data in a particular scope. This prevents data races.
// When the mutable reference goes out of scope, we can have another mutable reference to the same data
// Also we can't have a mutable reference while we have an immutable one
// Multiple immutable references are allowed

// Reference scopes are from the point at which they are introduced until the last point at which they are used

// We can't have a dangling reference, which is a reference that points to invalid memory. Compiler magic.


// The Rules of References:

// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.


fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Here we pass in a reference, indicated by the '&', so we don't move s1 to the function

    println!("The length of '{}' is {}.", s1, len); // This is possible because we passed in a reference to s1, so it's still in scope

    let mut s2 = String::from("hello");

    // We can pass in a mutable reference to a function, indicated by the '&mut'. For this to work, we also need to declare the variable as mutable
    change(&mut s2);
    
    // This won't work because s3 is immutable
    // change(s1); 

    println!("{}", s2);

    let reference_to_nothing = dangle(); // Not working, because the reference is pointing to invalid memory
}

fn calculate_length(s: &String) -> usize { // s takes a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.
  // Because we never had ownership of s1, we also don't need to return it here

fn change(random_string: &mut String) {
    random_string.push_str(", world");
}

fn dangle() -> &String { // If we would return the String directly, it would work. The ownership would get moved to the calling function
    let s = String::from("hello");

    &s // We return a reference to the String, but s goes out of scope and is dropped. This means that the reference is pointing to invalid memory
} // Rust prevents this by not allowing us to return a reference to a variable that goes out of scope