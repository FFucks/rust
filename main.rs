mod variables;
mod control_flow;
mod ownership;
mod structs;

fn main() {
    println!("== Variables ==");
    variables::run();

    println!("\n== Control Flow ==");
    control_flow::run();

    println!("\n== Ownership ==");
    ownership::run();

    println!("\n== Structs ==");
    structs::run();
}