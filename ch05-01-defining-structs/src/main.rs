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

// Since were using the = operator, the values are moved into the instance.
// If we have a look at user 4, this means we cannot use user3 email and username anymore.
// This is because the values were moved into user4. If we only specified to move sign_in_count and active, we could use user3 as normal.
// This is because u64 and bool implement the Copy trait, which means they are copied instead of moved.

// We can also define structs without named fields, called tuple structs.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// You can also define structs that don't have any fields, called unit-like structs.
// These are useful when you want to implement a trait on a type but don't have any data that you want to store in the type itself.
// They are defined like structs, but without any brackets.
struct UnitLikeStruct;

fn main() {
    // To use a struct, we need to instantiate it.
    // The values don't have to be in the same order as in the definition of the struct.
    let user1 = User {
        active : true,
        username : String::from("user1"),
        email : String::from("sample@mail.xy"),
        sign_in_count : 1,
    };

    // To get a specific value from a struct, we can use dot notation.
    // As seen below, we can also use the dot notation to change the value if the instance is mutable.
    // Note here, the entire instance must be mutable. Rust doesn't allow us to mark only certain fields as mutable.

    let mut mut_user2 = User {
        active : user1.active, // We can also use the dot notation to copy the value from another instance.
        username : String::from("user2"),
        email : String::from("mutsample@mail.xy"),
        sign_in_count : user1.sign_in_count,
    };

    mut_user2.active = false;

    let user3 = build_user(String::from("mail@mail.com"), String::from("user3"));

    let user4 = User {
        active : false,
        ..user3 // This is a shorthand for copying the remaining fields from user3. This must be the last field in the instance definition.
    };

    // Tuple structs are instantiated like tuples.
    // Even though they both are tuples of three integers, they are different types.
    // A function that takes a Color instance as a parameter will not accept a Point instance for example.
    // You can access the values of tuple structs using dot notation given the index of the value.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// We can also implicitly return instances of structs from functions.
fn build_user(email: String, username: String) -> User {
    User {
        active : true,
        username, // This is a shorthand for username : username, this only works if the variable name and the field name are the same.
        email,
        sign_in_count : 1,
    }
}