use crate::rect::Rect;
use crate::vec2::Vec2;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Array2<T>
where
    T: Clone + Default,
{
    data: Vec<Vec<T>>,
}

impl<T> Array2<T>
where
    T: Clone + Default,
{
    pub fn from_with_size(w: usize, h: usize) -> Self {
        Self {
            data: vec![vec![T::default(); w]; h],
        }
    }

    fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn line(&self, pos: Vec2<usize>, length: usize) -> &[T] {
        let Vec2(x, y) = pos;
        &self.data[y][x..(x + length)]
    }

    pub fn part_of(&self, rect: Rect) -> Self {
        let Vec2(w, h) = rect.size();
        let Vec2(x, y) = rect.pos();

        let mut result = Vec::new();
        for y in y..(y + h) {
            result.push(self.data[y][x..(x + w)].to_vec())
        }
        Array2::new(result)
    }
}

impl<T> Index<[usize; 2]> for Array2<T>
where
    T: Clone + Default,
{
    type Output = T;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[0]][index[1]]
    }
}

impl<T> IndexMut<[usize; 2]> for Array2<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[0]][index[1]]
    }
}

