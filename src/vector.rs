use std::fmt::{self, Display, Formatter};
use std::mem;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T, const N: usize>([T; N]);

pub type Vector2<T> = Vector<T, 2>;

impl<T: Copy + Default, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T: Copy + Default, const N: usize> Vector<T, N> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const N: usize> Vector<T, N> {
    #[must_use]
    pub fn into_inner(self) -> [T; N] {
        self.0
    }
}

impl<T: Display, const N: usize> Display for Vector<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut flag = false;
        self.0.iter().try_for_each(|i| {
            if mem::replace(&mut flag, true) {
                write!(f, ", {}", i)
            } else {
                write!(f, "{}", i)
            }
        })?;

        write!(f, "]")
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for Vector<T, N> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.iter_mut().for_each(|i| *i = -*i);
        self
    }
}

macro_rules! impl_binop {
    ($Op:ident :: $call:ident) => {
        impl<T: Copy + $Op<Output = T>, const N: usize> $Op<Vector<T, N>> for Vector<T, N> {
            type Output = Self;

            fn $call(mut self, rhs: Vector<T, N>) -> Self::Output {
                self.iter_mut().zip(rhs.into_iter()).for_each(|(l, r)| {
                    *l = $Op::$call(*l, r);
                });
                self
            }
        }

        impl<T: Copy + $Op<Output = T>, const N: usize> $Op<[T; N]> for Vector<T, N> {
            type Output = Self;

            fn $call(mut self, rhs: [T; N]) -> Self::Output {
                self.iter_mut().zip(rhs.into_iter()).for_each(|(l, r)| {
                    *l = $Op::$call(*l, r);
                });
                self
            }
        }

        impl<T: Copy + $Op<Output = T>, const N: usize> $Op<T> for Vector<T, N> {
            type Output = Self;

            fn $call(mut self, rhs: T) -> Self::Output {
                self.iter_mut().for_each(|lhs| *lhs = $Op::$call(*lhs, rhs));
                self
            }
        }
    };
}

macro_rules! impl_binop_assign {
    ($Op:ident :: $call:ident) => {
        impl<T: Copy + $Op, const N: usize> $Op<Vector<T, N>> for Vector<T, N> {
            fn $call(&mut self, rhs: Vector<T, N>) {
                self.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(l, r)| $Op::$call(l, r));
            }
        }

        impl<T: Copy + $Op, const N: usize> $Op<[T; N]> for Vector<T, N> {
            fn $call(&mut self, rhs: [T; N]) {
                self.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(l, r)| $Op::$call(l, r));
            }
        }

        impl<T: Copy + $Op, const N: usize> $Op<T> for Vector<T, N> {
            fn $call(&mut self, rhs: T) {
                self.iter_mut().for_each(|lhs| $Op::$call(lhs, rhs));
            }
        }
    };
}

impl_binop!(Add::add);
impl_binop!(Sub::sub);
impl_binop!(Mul::mul);
impl_binop!(Div::div);
impl_binop!(Rem::rem);

impl_binop_assign!(AddAssign::add_assign);
impl_binop_assign!(SubAssign::sub_assign);
impl_binop_assign!(MulAssign::mul_assign);
impl_binop_assign!(DivAssign::div_assign);
impl_binop_assign!(RemAssign::rem_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn V2<T>(x: T, y: T) -> Vector2<T> {
        Vector2::from([x, y])
    }

    #[test]
    fn binops() {
        let sut = Vector2::default();

        assert_eq!(sut + V2(1, 1), [1, 1].into());
        assert_eq!(sut + [1, 1], [1, 1].into());
        assert_eq!(sut + 1, [1, 1].into());

        assert_eq!(sut - V2(1, 1), [-1, -1].into());
        assert_eq!(sut - [1, 1], [-1, -1].into());
        assert_eq!(sut - 1, [-1, -1].into());

        assert_eq!(sut * V2(1, 1), [0, 0].into());
        assert_eq!(sut * [1, 1], [0, 0].into());
        assert_eq!(sut * 1, [0, 0].into());

        assert_eq!((sut + 4) * V2(-1, -1), [-4, -4].into());
        assert_eq!((sut + 4) * [-1, -1], [-4, -4].into());
        assert_eq!((sut + 4) * -1, [-4, -4].into());

        assert_eq!(sut / V2(1, 1), [0, 0].into());
        assert_eq!(sut / [1, 1], [0, 0].into());
        assert_eq!(sut / 1, [0, 0].into());

        assert_eq!((sut + 4) / V2(-2, -2), [-2, -2].into());
        assert_eq!((sut + 4) / [-2, -2], [-2, -2].into());
        assert_eq!((sut + 4) / -2, [-2, -2].into());

        assert_eq!(sut % V2(2, 2), [0, 0].into());
        assert_eq!(sut % [2, 2], [0, 0].into());
        assert_eq!(sut % 2, [0, 0].into());

        assert_eq!((sut + 3) % V2(2, 2), [1, 1].into());
        assert_eq!((sut + 3) % [2, 2], [1, 1].into());
        assert_eq!((sut + 3) % 2, [1, 1].into());
    }
}
