pub trait XRange<T>: std::ops::RangeBounds<T>
    where T:  std::ops::Add<T>
{
    fn offset(&self, offset: T) -> std::ops::Range<T>;
    fn inside(&self, other: &std::ops::Range<T>) -> bool;
}

//
// impl XRange<usize> for std::ops::Range<usize> {
//     fn offset(&self, offset: isize) -> std::ops::Range<usize> {
//         self.start.checked_add_signed(offset)
//         (self.start as isize- offset) as usize
//     // (self.start+offset)..(self.end+offset)
//     }
//
//     fn inside(&self, other: &std::ops::Range<usize>) -> bool {
//         self.contains(&other.start) && self.contains(&other.end)
//     }
// }

