pub fn largest<T>(vec: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    let mut large: &T = &vec[0];
    for item in vec {
        if item > large {
            large = item;
        }
    }

    large
}
