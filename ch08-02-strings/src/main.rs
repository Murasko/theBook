fn main() {
    let mut s = String::new();

    let data = "Initial Names";
    let s = data.to_string();

    let s = "Initial Names".to_string();
    // equivalent would be
    let s = String::from("Initial Names");

    // + operator can be used to concatenate strings
    let mut s = String::from("first value");
    s.push_str(" second value");
    println!("Value of s is {}", s);

    let mut s = String::from("first value");
    let v2 = " second value";
    s.push_str(v2);
    print!("Value of s is {}", s);

    let mut n = String::from("nic");
    n.push('e');
    println!("Value of n is {}", n);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("Value of s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
}
