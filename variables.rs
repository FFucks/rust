pub fn run() {
    let x = 5;
    let mut y = 10;

    y += 5;

    println!("x = {x}, y = {y}");

    let x = x + 1;
    println!("shadowed x = {x}");

    let a: i32 = 10;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';

    println!("{a}, {b}, {c}, {d}");

    let tup = (1, 2.5, 'a');
    let (x, y, z) = tup;

    println!("tuple: {x}, {y}, {z}");

    let arr = [1, 2, 3];
    println!("array first: {}", arr[0]);
}