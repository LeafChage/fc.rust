pub trait RangeInside<T>: std::ops::RangeBounds<T> {
    fn inside(&self, other: &Self) -> bool;
}

impl<T> RangeInside<T> for std::ops::Range<T>
where
    T: Ord,
{
    fn inside(&self, other: &Self) -> bool {
        self.contains(&other.start) && self.contains(&other.end)
    }
}
