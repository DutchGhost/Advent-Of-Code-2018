pub trait Merge<U: ?Sized> {
    fn can_merge(&self, with: &U) -> bool;
}

/// T: Merge<U>, => T: Merge<[U]>
impl<T, U> Merge<[U]> for T
where
    T: Merge<U>,
{
    fn can_merge(&self, with: &[U]) -> bool {
        with.iter().any(|w| self.can_merge(w))
    }
}

/// T: Merge<W> => [T]: Merge<[W]>
impl<T, U> Merge<[U]> for [T]
where
    T: Merge<U>,
{
    fn can_merge(&self, with: &[U]) -> bool {
        self.iter()
            .any(|point| <T as Merge<[U]>>::can_merge(point, with))
    }
}
