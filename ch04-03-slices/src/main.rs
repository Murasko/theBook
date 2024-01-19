// Slices are kind of references, they also don't have ownership.
// They let you reference a contiguous sequence of elements in a collection rather than the whole collection.

// String literals are slices. Thats why they are immutable, &str is an immutable reference.

// Slices are not only for strings, they can be used for arrays or other collections as well.
// The syntax for slices is [starting_index..ending_index], where starting_index is inclusive and ending_index is exclusive.

fn main() {
    let mut s1 = String::from("hello world");

    let word = first_word_v1(&s1);

    s1.clear();

    // word still has the value 5 here, but there's no more string that we could meaningfully use the value 5 with.
    // word is now totally invalid!
    // This is because word isn't connected to the state of s.
    println!("The first word from {} is ending at index {}", s1, word);
    // If we would write a second word function now, we would need to keep even more stuff in mind, with the start and end index.

    let mut s2 = String::from("hello world"); // Since we cannot use clear() anymore, we could also remove the mut keyword here

    let word = first_word_v2(&s2);

    // We can't use clear() here, because we have an immutable reference (word) to s2. As learned in previous section, we cannot have both mutable and immutable references in the same scope.
    // s2.clear();

    println!("The first word from {} is {}", s2, word);
}

fn first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert the string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() { // Iterate over the array of bytes
        if item == b' ' { // If the current item is a space
            return i; // Return the index of the space
        }
    }

    s.len() // If there is no space, return the length of the string
}

// To fix the above problem, we can use a string slice
fn str_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // To make the above code more readable, we can use the following syntax
    // If we want to use the first or last index, we can omit the number
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
}

fn first_word_v2(s: &String) -> &str { // Return a string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // If the current item is a space
            return &s[0..i]; // Return a string slice from the start of the string to the index of the space
        }
    }

    &s[..]
}

// The last edit we can make to the first_word function is to change the input parameter type to &str
// This way we can use the function on both String and &str values, basically we could use it on String references or String literals (which are already slices)
fn first_word_v3(s: &str) -> &str { // Return a string slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // If the current item is a space
            return &s[0..i]; // Return a string slice from the start of the string to the index of the space
        }
    }

    &s[..]
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // This is a slice of type &[i32]
}