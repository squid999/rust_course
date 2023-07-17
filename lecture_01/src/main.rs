mod loop_01;
mod loop_02;

use loop_02::print_fn;

fn main() {
    println!("submod loop1 print ['a'..'Z']");
    loop_01::print();
    println!();

    println!("submod loop2 print ['A'..'z']");
    print_fn::print_pretty();
    println!();
}

// fn sample1() {
//     let s1 = String::from("run");
//     let s2 = &s1;
//     println!("{}", s2);
//     s2.push_str("xxx"); // 错误，禁止修改租借的值
//     println!("{}", s2);
//     let s3 = s1; // 错误，s1 的所有权被
//     println!("{}", s2);
//     s2 = &s3;
//     println!("{}", s2);
// }
