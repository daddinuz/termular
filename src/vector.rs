use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector<T, const N: usize>([T; N]);

pub type Vector2<T> = Vector<T, 2>;
pub type Vector3<T> = Vector<T, 3>;

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}

impl<T: Default + Copy, const N: usize> Vector<T, N> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self([value.0, value.1])
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self([value.0, value.1, value.2])
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

impl<T: Copy + Add<Output = T>, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.iter_mut().zip(rhs.iter()).for_each(|(l, r)| {
            *l = *l + *r;
        });
        self
    }
}

impl<T: Copy + AddAssign, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(l, r)| *l += *r);
    }
}

impl<T: Copy + Sub<Output = T>, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.iter_mut().zip(rhs.iter()).for_each(|(l, r)| {
            *l = *l - *r;
        });
        self
    }
}

impl<T: Copy + SubAssign, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.iter_mut().zip(rhs.iter()).for_each(|(l, r)| *l -= *r);
    }
}

impl<T: Copy + Mul<Output = T>, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.iter_mut().for_each(|lhs| *lhs = *lhs * rhs);
        self
    }
}

impl<T: Copy + MulAssign, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|lhs| *lhs *= rhs);
    }
}

impl<T: Copy + Div<Output = T>, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(mut self, rhs: T) -> Self::Output {
        self.iter_mut().for_each(|lhs| *lhs = *lhs / rhs);
        self
    }
}

impl<T: Copy + DivAssign, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        self.iter_mut().for_each(|lhs| *lhs /= rhs);
    }
}

impl<T: Copy> Vector2<T> {
    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }
}

impl<T> Vector2<T> {
    pub fn x_ref(&self) -> &T {
        &self[0]
    }

    pub fn y_ref(&self) -> &T {
        &self[1]
    }
}

impl<T> Vector2<T> {
    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }
}

impl<T: Copy> Vector3<T> {
    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }
}

impl<T> Vector3<T> {
    pub fn x_ref(&self) -> &T {
        &self[0]
    }

    pub fn y_ref(&self) -> &T {
        &self[1]
    }

    pub fn z_ref(&self) -> &T {
        &self[2]
    }
}

impl<T> Vector3<T> {
    pub fn x_mut(&mut self) -> &mut T {
        &mut self[0]
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self[1]
    }

    pub fn z_mut(&mut self) -> &mut T {
        &mut self[2]
    }
}
