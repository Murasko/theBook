// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// This works fine, but you cannot clearly see the relation of the parameters from area and the rectangle that you calculate.
// For the next step we refactor the code to make the use of tuples.

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
// This makes it better so we only need to pass one variable and have a bit more structure. On the other side it removes clarity in the calculation
// since we now need to remember which param is which one. Not needed for the calculation, could mess up hard time if you wanna for example draw out the rectangle.
// Next refactoring will make use of structs to label data and make it overall more clear to use.

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     println!("rect1 is {rect1:#?}");
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// With the debug macro you will print the output to the stderr instead of stdout. Also you will see the exact place where your code prints.
// The debug macro also takes ownership and returns it afterwards instead of only taking a reference like the println macro does.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
