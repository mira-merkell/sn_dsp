use std::{
    fmt::Debug,
    iter::Sum,
    ops::{
        Add,
        AddAssign,
        Index,
        IndexMut,
        Mul,
        MulAssign,
        Neg,
        Sub,
        SubAssign,
    },
};

use super::Frame;
use crate::num::{
    Float,
    Zero,
};

/// Array frame
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arf<T, const N: usize>([T; N]);

impl<T, const N: usize> From<[T; N]> for Arf<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T, const N: usize> From<Arf<T, N>> for [T; N] {
    fn from(value: Arf<T, N>) -> Self {
        value.0
    }
}

impl<T, const N: usize> Index<usize> for Arf<T, N> {
    type Output = T;

    fn index(
        &self,
        index: usize,
    ) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Arf<T, N> {
    fn index_mut(
        &mut self,
        index: usize,
    ) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const N: usize> Zero for Arf<T, N>
where
    T: Float,
{
    fn zero() -> Self {
        Self([T::zero(); N])
    }
}

impl<T, const N: usize> Default for Arf<T, N>
where
    T: Float,
{
    fn default() -> Self {
        Self::zero()
    }
}

impl<T, const N: usize> Add for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] + rhs.0[i];
        }
        out
    }
}

impl<T, const N: usize> AddAssign for Arf<T, N>
where
    T: Float,
{
    fn add_assign(
        &mut self,
        rhs: Self,
    ) {
        for i in 0..N {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<T, const N: usize> Sub for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] - rhs.0[i];
        }
        out
    }
}

impl<T, const N: usize> SubAssign for Arf<T, N>
where
    T: Float,
{
    fn sub_assign(
        &mut self,
        rhs: Self,
    ) {
        for i in 0..N {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<T, const N: usize> Neg for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = -self.0[i];
        }
        out
    }
}

impl<T, const N: usize> Mul<T> for Arf<T, N>
where
    T: Float,
{
    type Output = Self;

    fn mul(
        self,
        rhs: T,
    ) -> Self::Output {
        let mut out = Self::zero();
        for i in 0..N {
            out.0[i] = self.0[i] * rhs;
        }
        out
    }
}

impl<T, const N: usize> MulAssign<T> for Arf<T, N>
where
    T: Float,
{
    fn mul_assign(
        &mut self,
        rhs: T,
    ) {
        for i in 0..N {
            self.0[i] *= rhs;
        }
    }
}

impl<T, const N: usize> Sum for Arf<T, N>
where
    T: Float,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, frame| acc + frame)
    }
}

impl<T, const N: usize> Frame for Arf<T, N>
where
    T: Float,
{
    type Sample = T;

    fn as_slice(&self) -> &[Self::Sample] {
        &self.0
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Sample] {
        &mut self.0
    }
}

impl<T> Arf<T, 2>
where
    T: Float,
{
    pub fn flip(&mut self) {
        let lf = self[0];
        let rf = self[1];
        self[0] = rf;
        self[1] = lf;
    }
}
