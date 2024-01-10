/// fills elements of vector with `initial` value.
///
/// # Arguments
///
/// * `vec` - The vector to perform fill.
///
/// * `initial` - The value to be filled.
///
/// # Returns
///
/// Returns the new filled vector.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// let vec1 = vec![0; 5];
///
/// assert_eq!(vec![0, 0, 0, 0, 0], vec1);
///
/// assert_eq!(vec![1, 1, 1, 1, 1], collection::fill(vec1, 1));
///
/// ```

pub fn fill<T: Clone>(vec: Vec<T>, initial: T) -> Vec<T> {
    vec.into_iter().map(|_| initial.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        let vec1 = vec![0; 5];
        assert_eq!(vec![0, 0, 0, 0, 0], vec1);
        assert_eq!(vec![1, 1, 1, 1, 1], fill(vec1, 1));
    }
}
