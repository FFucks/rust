pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s2 = {s2}");

    let s3 = String::from("world");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    let s = String::from("rust");
    print_str(&s);

    let mut s = String::from("rust");
    mutate(&mut s);

    println!("mutated: {s}");
}

fn print_str(s: &String) {
    println!("borrowed: {s}");
}

fn mutate(s: &mut String) {
    s.push_str(" lang");
}