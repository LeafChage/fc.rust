#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> {
    pub fn new(a: T, b: T) -> Self {
        Vec2(a, b)
    }
}

impl Vec2<usize> {
    pub fn one() -> Self {
        Vec2(1, 1)
    }

    pub fn zero() -> Self {
        Vec2(0, 0)
    }
}

impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Vec2(l1, l2) = self;
        let Vec2(r1, r2) = rhs;
        Vec2(l1 - r1, l2 - r2)
    }
}

impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Vec2(l1, l2) = self;
        let Vec2(r1, r2) = rhs;
        Vec2(l1 + r1, l2 + r2)
    }
}

impl<T, E> std::ops::Div<E> for Vec2<T>
where
    E: Copy,
    T: std::ops::Div<E, Output = T>,
{
    type Output = Self;
    fn div(self, rhs: E) -> Self::Output {
        let Vec2(l1, l2) = self;
        Vec2(l1 / rhs, l2 / rhs)
    }
}
