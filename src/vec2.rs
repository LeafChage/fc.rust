#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec2<T>
where
    T: Clone + Copy,
{
    x: T,
    y: T,
}

impl<T> Vec2<T>
where
    T: Clone + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn xy(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Vec2<usize> {
    pub fn one() -> Self {
        Vec2::new(1, 1)
    }

    pub fn zero() -> Self {
        Vec2::new(0, 0)
    }
}

impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
    T: Clone + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> std::ops::Add for Vec2<T>
where
    T: Clone + Copy,
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T, E> std::ops::Div<E> for Vec2<T>
where
    E: Copy,
    T: std::ops::Div<E, Output = T>,
    T: Clone + Copy,
{
    type Output = Self;
    fn div(self, rhs: E) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T, E> std::ops::Mul<E> for Vec2<T>
where
    E: Copy,
    T: std::ops::Mul<E, Output = T>,
    T: Clone + Copy,
{
    type Output = Self;
    fn mul(self, rhs: E) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
