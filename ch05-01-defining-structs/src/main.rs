// Structs are similar to tuples. They are a way to group related data together.
// Unlike tuples, structs have names associated with their data.
// Structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

// Definition of structs is relatively easy
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // To use a struct, we need to instantiate it.
    // The values don't have to be in the same order as in the definition of the struct.
    let user1 = User {
        active : true,
        username : String::from("user1"),
        email : String::from("sample@mail.xy"),
        sign_in_count : 1,
    }
}
