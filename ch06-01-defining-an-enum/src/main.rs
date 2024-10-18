// Structs group multiple properties of one thing (example the height and width of the rectangle). Enums groups set of values (example rectangle in a group of others like circle and triangle).
// An enum can only be one of the given variants.
// Enums are defined by the enum keyword followed by a name, and the variants in curly brackets. The variants are now new data types.
// You can create instances of these with the namespace syntax.
// Each Enum variant is at the same time also a constructor (IpAddr::V4() takes a string and returns an instance of IpAddr) 
// By not using a struct to map values to the enums we can also have different types in the variants like shown in the IpAddr example

// The message enum hast four variants with 4 different types. Enum variants can be any kind of data.
// The enum definition would be the same as the following struct definitions. The main difference here is the possibility to use impl on the enum to share it across all. This wouldn't be possible if we wouldv'e used the structs.
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let home = IpAddr::V4(String::from(127, 0, 0, 1));
let loopback = IpAddr::V6(String::from("::1"));

// Option Enum
// Rust doesn't have NULL values, it has the Option enum which is either something or nothing but must be one. The following is the definition of the Option<T> enum.
enum Option<T> {
    None,
    Some(T),
}
// The option enum, some and none are all loaded in the prelude, so there is no need of bringing it into scope or use Option::Some
// <T> can hold one piece data of any type and manipulates `Some` based on it's input. Examples:
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;

// absent_number cannot be inferred because the compiler doesn't know what value there should be. 