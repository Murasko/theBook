// The match control flow allows you to compare a value against a predefined set of patterns. The first match will stop the execution and comparison to the other patterns.
// To use the match statement, start with the match keyword, followed by the expression. Compared to the if statement which needs an bool as expression, match can use any type, in this case an enum, as expression.
// The match arms has two parts, pattern and value. They are seperated by a comma.
// If you use multiple lines of code in the match arm, you need to use curly brackets. The comma to seperate will be optional

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value is {} cents", value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let test = Some("Test");
    println!("five {five:#?}, six {six:#?}, none {none:#?}, huh {test:#?}")
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


// Catch all matches

let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}


let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}


let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
