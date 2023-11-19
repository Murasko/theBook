fn main() {
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = x + 1;

    // x = 8; <-- Wouldn't work here because we shadowed intial x with let x and without mut

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");

    }

    println!("The value of x is: {x}");


    let spaces = "           ";
    let spaces = spaces.len();

    println!("Number of spaces: {spaces}");

    // Not working:
    // let spaces = "     ";
    // spaces = spaces.len();
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
