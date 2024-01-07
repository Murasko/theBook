fn main() {
    println!("Hello, world!");
    // if_expression();
    // else_if_expression();
    // if_in_let();
    // loop_loop();
    // result_loop();
    // label_loops();
    // while_loop();
    // for_loop();
    for_loop_range();
}


// if expressions
// Blocks of code associated with the conditions in if expressions are sometimes called arms, just like the arms in match expressions.
// Else expressions are optional, if not provided, the program will just skip the if block.
// The condition must be a bool, if not, the program will not compile.
fn if_expression() {
    let condition = false;

    if condition {
        println!("condition is true");
    } else {
        println!("condition is false");
    }
}

// else if expressions
// You can have multiple else if expressions, the program will execute the first block that matches the condition.
// Using too many (more than one) else if expressions is bad practice. Consider using match expressions instead.
fn else_if_expression() {
    let number = 1;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Using if in a let statement
// Since if is an expression, it can be assigned to a variable via let.
// The type of the variable must be the same for both arms of the if expression.
// It is not possible to assign a int or a string (true / false if expression) to a variable.
fn if_in_let() {
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}


// Loooops
// Rust has three kinds of loops: loop, while, and for.
// You can use the 'break' keyword to exit a loop and the 'continue' keyword to skip the rest of the current iteration and start a new one.
fn loop_loop() {
    loop {
        print!("The Looooops! ");
    }
}

// You can return a value from a loop by adding the value after the break keyword.
fn result_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // The break keyword returns a value from the loop that can be used in the rest of the program.
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// You can label loops if you nest them to specify which loop to break or continue.
// The label must be followed by a colon and begin with a single quote.
fn label_loops() {
    let mut counter = 0;
    let mut stuff = 0;
    'count_up: loop {
        println!("Counter: {}", counter);
        loop {
            println!("Do stuff: {}", stuff);     
            if stuff == counter * 2 {
                break;
            }
            if counter == 10 {
                break 'count_up;
            }
            stuff += 2; 
        }
        counter += 1;
    }
    println!("Done");
}

// While loops
// Nothing much to say i guess.
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// For loops
// Same like the while loop, nothing much to say.
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// You can also use for loops to iterate over a range of numbers, provided by the standard library.
// The range is inclusive on the lower bound and exclusive on the upper bound.
fn for_loop_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}