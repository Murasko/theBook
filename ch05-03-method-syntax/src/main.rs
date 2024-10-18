// Methods are similar to functions, both declared by fn keyword followed by a name, both can have parameters and return values, both contain code that is run when called
// Key difference, methods are defined within the context of a struct (or enum or trait object) and their first parameter is always `self`, which represents the instance from where its called from

// To define a function in the context of our struct, we use the `impl` block for our structname.
// After we've moved our function in the impl block, we replace all references with `self`. `&self` is here actually short for `self: &Self`.
// To access this method later on, we access the struct, followed by a dot, the method name, parentheses and any arguments.
// Methods can take ownership of `self`, borrow `self` immutably, as we’ve done here, or borrow `self` mutably, just as they can any other parameter. If we wanted to change data we would've used `&mut self`.

// All functions in the impl block are called `associated functions`. We can also specify associated functions without reference to Self because they don't need no reference to it. These are often `constructors` and are called `new`.
// New isn't a special name, so it's not enforced to name them like that. We could for example create a square by just typing in one parameter instead of two by creating a constructor without self as parameter.
// To call a associated function that isn't a method you use the syntax: name::function. This is the same syntax also for namespaces created by modules.

// It's possible and correct syntax to have multiple impl blocks per struct.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq1 = Rectangle::square(5);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("The area of the square is {} square pixels.", sq1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("The outer diameter of the square is {}", sq1.height * 4);
}

// You could implement a method with the same name as the struct field. If we want to just return it, this is also called a getter. Rust doesn't implement these by default, but they are useful
// to make the method public but the attribute private for example.
// Example:

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn width(&self) -> u32 {
//         self.width
//     }
// }
