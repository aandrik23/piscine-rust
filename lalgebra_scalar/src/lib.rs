use std::{
    iter::Sum,
    ops::{Add, Mul},
};

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i32 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

impl Scalar for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        Some(Vector(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(l, r)| l + r)
                .collect(),
        ))
    }
}

impl<T> Vector<T>
where
    T: Scalar + Sum<T>,
{
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        Some(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(x, y)| x * y)
                .sum(),
        )
    }
}