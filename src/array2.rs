use crate::rect::Rect;
use crate::vec2::Vec2;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Eq)]
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

    /// (width, height)
    pub fn dimention(&self) -> Vec2<usize> {
        Vec2::new(self.data[0].len(), self.data.len())
    }

    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn line(&self, pos: Vec2<usize>, length: usize) -> &[T] {
        let (x, y) = pos.xy();
        &self.data[y][x..(x + length)]
    }

    pub fn part_of(&self, rect: Rect) -> Self {
        let (w, h) = rect.size().xy();
        let (x, y) = rect.pos().xy();

        let mut result = Vec::new();
        for y in y..(y + h) {
            result.push(self.data[y][x..(x + w)].to_vec())
        }
        Array2::new(result)
    }

    pub fn put_array2(&mut self, pos: Vec2<usize>, value: Array2<T>) {
        let mut offset_y = 0;
        for line in value.data.into_iter() {
            let mut offset_x = 0;
            for value in line.into_iter() {
                let pos = pos + Vec2::new(offset_x, offset_y);
                self[pos] = value;
                offset_x += 1;
            }
            offset_y += 1;
        }
    }
}

impl<T> Index<Vec2<usize>> for Array2<T>
where
    T: Clone + Default,
{
    type Output = T;
    /// x, y
    fn index(&self, index: Vec2<usize>) -> &Self::Output {
        let (x, y) = index.xy();
        &self.data[y][x]
    }
}

impl<T> IndexMut<Vec2<usize>> for Array2<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, index: Vec2<usize>) -> &mut Self::Output {
        let (x, y) = index.xy();
        &mut self.data[y][x]
    }
}

impl<T> Index<[usize; 2]> for Array2<T>
where
    T: Clone + Default,
{
    type Output = T;
    /// x, y
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.data[index[1]][index[0]]
    }
}

impl<T> IndexMut<[usize; 2]> for Array2<T>
where
    T: Clone + Default,
{
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.data[index[1]][index[0]]
    }
}

impl<T> std::fmt::Debug for Array2<T>
where
    T: Clone + Default + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for line in self.data.iter() {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}

#[test]
fn it_put_array2() {
    let mut a = Array2::from_with_size(10, 10);
    a.put_array2(Vec2::new(3, 3), Array2::new(vec![vec![1; 2], vec![1; 2]]));
    assert_eq!(
        a,
        Array2::new(vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ])
    )
}

#[test]
fn it_part_of() {
    let a = Array2::new(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    let b = a.part_of(Rect::from_pos_size(Vec2::new(3, 3), Vec2::new(2, 2)));
    assert_eq!(b, Array2::new(vec![vec![1, 1], vec![1, 1],]))
}
