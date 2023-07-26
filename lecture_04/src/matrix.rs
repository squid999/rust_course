// Matrix

use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Default + Debug,
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| T::default()))
    }

    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a T;
    type IntoIter = ::std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut T;
    type IntoIter = ::std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> Add<Matrix<T>> for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Copy,
{
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.rows == rhs.rows);
        assert!(self.cols == rhs.cols);

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .into_iter()
                .zip(rhs.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}

pub trait Printable {
    fn print(&self);
}

impl<T> Printable for Matrix<T>
where
    T: Default + Debug,
{
    fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{:4?} ", self.data[i * self.rows + j])
            }
            println!()
        }
    }
}

pub fn print_pretty(matrix: &dyn Printable) {
    matrix.print()
}
