fn main() {
    let float64 = 2.0; // double precision
    let float32: f32 = 3.0; // single precision
    println!("Answer: {float64} // {float32}");

    // Numeric Operations

    // addition
    let sum = 10.5 + 10.123;
    println!("Answer: {sum}");

    // subtraction
    let difference = 100.23 - 45.12;
    println!("Answer: {difference}");

    // multiplication
    let product = 4 * 15;
    println!("Answer: {product}");

    // division
    let quotient = 55.3 / 12.5;
    let truncated = -9 / 2;
    println!("Answer: {quotient} // {truncated}");

    // remainder
    let remainder = 48 % 9;
    println!("Answer: {remainder}");

    // Booleans

    let _t = true;
    let _f: bool = false;

    // Char, single quotes
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Tuples
    let _tup: (i32, f64, u8) = (-500, 6.4, 1);
    let (x, y, z) = _tup;
    println!("The values are: {z}, {x}, {y}");

    let _one = _tup.2;

    // Arrays
    let _a = [1, 2, 3, 4, 5, 6];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let _a = [3; 5]; // ==> let a = [3, 3, 3, 3, 3];

    let _first = _a[0];
    let _second = _a[1];

}
