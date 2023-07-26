mod matrix;
mod order_enum;
mod order_trait_object;

use matrix::Matrix;

fn project01() {
    println!("\nshow enum vector case:");
    order_enum::poll_orders();
    println!("\nshow trait object case:");
    order_trait_object::poll_orders();
}

fn project02() {
    let m1: Matrix<i32> = Matrix::new(3, 3);

    println!("{:?}", m1);
    println!("\nmatrix1 rows: {} cols: {}", m1.rows(), m1.cols());
    let m2: Matrix<u32> = Matrix::from_iter(5, 6, 0..);

    println!("\nmatrix02: \n{:?}", m2);
    let m3 = m2.clone();
    println!("\nmatrix03(clone from m2): \n{:?}", m3);

    let sum1 = m2 + m3;

    println!("\nshow sum of matrix2 and matrix3: \n{:?}", sum1);

    println!("\nprint matrix3 pretty");
    matrix::print_pretty(&sum1);
}
fn main() {
    println!("Hello, Lecture04!");

    project01();
    project02();
}
