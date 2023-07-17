mod loop_01;
mod loop_02;

fn main() {
    println!("submod loop1 print ['a'..'Z']");
    loop_01::print();
    println!();

    println!("submod loop2 print ['A'..'z']");
    loop_02::print_fn::print_pretty();
    println!();
}
