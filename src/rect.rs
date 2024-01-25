use crate::vec2::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pos: Vec2<usize>,
    size: Vec2<usize>,
}

impl Rect {
    pub fn from_pos2(pos: Vec2<usize>, max_pos: Vec2<usize>) -> Self {
        Self {
            pos,
            size: max_pos - pos,
        }
    }

    pub fn from_pos_size(pos: Vec2<usize>, size: Vec2<usize>) -> Self {
        Self { pos, size }
    }
    pub fn x(&self) -> usize {
        self.pos.0
    }
    pub fn y(&self) -> usize {
        self.pos.1
    }
    pub fn width(&self) -> usize {
        self.size.0
    }
    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn size(&self) -> Vec2<usize> {
        self.size
    }

    pub fn pos(&self) -> Vec2<usize> {
        self.pos
    }

    pub fn max_pos(&self) -> Vec2<usize> {
        self.size() + self.pos()
    }
}

impl std::ops::Div<usize> for Rect {
    type Output = Rect;

    fn div(self, rhs: usize) -> Self::Output {
        Self::from_pos_size(self.pos(), self.size() / rhs)
    }
}
