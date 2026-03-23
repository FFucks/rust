pub fn run() {
    let x = 7;

    if x > 5 {
        println!("bigger than 5");
    } else {
        println!("less or equal to 5");
    }

    let mut count = 0;

    loop {
        count += 1;

        if count == 3 {
            break;
        }
    }

    println!("loop finished in {count}");

    let mut n = 0;
    while n < 3 {
        n += 1;
    }

    println!("while finished in {n}");

    for i in 1..4 {
        println!("for: {i}");
    }
}