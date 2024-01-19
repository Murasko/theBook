fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    multiple_parameters(5, 'h');

    let x = 5; // statement

    // As you can see below, the expression does not include the semicolon. If it does, it becomes a statement.
    
    let y = { 
        let x = 3;
        x + 1 // expression
    };
}

fn another_function() {
    println!("Another function!");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn multiple_parameters(x: i32, label: char) {
    println!("The value of x is: {x}{label}");
}

// Functions with return values don't name return values, but instead, name the type of the value returned after an arrow (->)
fn return_value_function(x: i32) -> i32 {
    x + 1
}

// We can also use the return keyword to return early from a function
fn return_early_function(x: i32) -> i32 {
    return x + 1;
    x + 2
}


// If we don't specify a return value, the function will return the last expression implicitly.
// This also means, we get an error if we don't have an expression at the end of the function.
fn implicit_return_function(x: i32) -> i32 {
    x + 1
    // x + 2; <-- This will cause an error if we wouldn't have the line above
}
