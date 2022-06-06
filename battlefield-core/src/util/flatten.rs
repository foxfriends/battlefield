pub(crate) struct Flatten<T>(pub T);

impl<T, U> FromIterator<U> for Flatten<T>
where
    U: IntoIterator,
    T: FromIterator<<U as IntoIterator>::Item>,
{
    fn from_iter<I: IntoIterator<Item = U>>(iter: I) -> Self {
        Flatten(iter.into_iter().flatten().collect())
    }
}
