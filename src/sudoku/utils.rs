/*pub fn simplify(xs: &[Option<char>]) -> String {
    xs.iter()
        .map(|x| match x {
            Some(v) => *v,
            None => '-',
        }).fold(String::new(), |mut acc, x| {
            acc.push(x);
            acc
        })
}*/
pub fn splice<T>(xs: &[Option<T>], y: T) -> Vec<Option<T>>
where
    T: Clone,
{
    use std::iter;
    let mut y_iter = iter::once(y);
    xs.iter()
        .map(|x| match x {
            Some(value) => Some((*value).clone()),
            None => y_iter.next(),
        })
        .collect::<Vec<Option<T>>>()
}
pub fn append<T>(xs: &[T], y: &T) -> Vec<T>
where
    T: Clone,
{
    use std::iter;
    let y_iter = iter::once(y);
    xs.into_iter().chain(y_iter).cloned().collect::<Vec<T>>()
}
