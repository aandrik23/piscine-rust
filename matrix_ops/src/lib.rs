use std::ops::{Add, Mul, Sub};

use lalgebra_scalar::Scalar;
use matrix::Matrix;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    fn from(value: [[T; W]; H]) -> Self {
        Wrapper(Matrix(value))
    }
}

impl<const W: usize, const H: usize, T> Add for Wrapper<W, H, T>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::zero();

        for y in 0..H {
            for x in 0..W {
                result.0[y][x] = self.0.0[y][x] + rhs.0.0[y][x];
            }
        }

        Wrapper(result)
    }
}

impl<const W: usize, const H: usize, T> Sub for Wrapper<W, H, T>
where
    T: Scalar + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::zero();

        for y in 0..H {
            for x in 0..W {
                result.0[y][x] = self.0.0[y][x] - rhs.0.0[y][x];
            }
        }

        Wrapper(result)
    }
}

impl<const S: usize, T> Mul for Wrapper<S, S, T>
where
    T: Scalar + Add<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Matrix::zero();

        for y in 0..S {
            for x in 0..S {
                let mut sum = T::zero();

                for i in 0..S {
                    sum = sum + self.0.0[y][i] * rhs.0.0[i][x];
                }

                result.0[y][x] = sum;
            }
        }

        Wrapper(result)
    }
}