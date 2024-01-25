use std::ops::Range;

pub fn inside<R, RT>(r: R, other: &Range<RT>) -> bool
where
    R: std::ops::RangeBounds<RT>,
    RT: PartialOrd,
{
    r.contains(&other.start) && r.contains(&other.end)
}

pub fn add<T: std::ops::Add + Copy>(r: Range<T>, offset: T) -> Range<T::Output> {
    (r.start + offset)..(r.end + offset)
}

pub fn sub<T: std::ops::Sub + Copy>(r: Range<T>, offset: T) -> Range<T::Output> {
    (r.start - offset)..(r.end - offset)
}
