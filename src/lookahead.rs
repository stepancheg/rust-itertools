
/// Iterator that can be queried if next element is present.
pub trait LookaheadIterator : Iterator
{
    /// Return `true` iff call to `next()` will returns `Some(..)`
    ///
    /// Call may modify iterator state. Thus this function accepts
    /// mutable self.
    fn has_next(&mut self) -> bool;
}
