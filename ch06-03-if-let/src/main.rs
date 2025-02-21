fn main() {
    println!("Hello, world!");

    let config_max = Some(7u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
        print_num(&max);
    }
}

fn print_num(refer: &u8) {
    println!("Number is {}", refer);
}
