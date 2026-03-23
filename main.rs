mod variables;
mod control_flow;

fn main() {
    println!("== Variables ==");
    variables::run();

    println!("\n== Control Flow ==");
    control_flow::run();
}