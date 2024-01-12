/// Returns true if all elements of the collection pass the predicate function check.
///
/// * predicate function signature: ```fn(item: &T, index: usize) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The function invoked per iteration.
///
/// # Returns
///
/// Returns true if all elements pass the predicate check, else false.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(true, collection::all_match(&[1, 2, 3], &|n: &i32, _i: usize| *n <= 3));
///
/// assert_eq!(false, collection::all_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n % 2 == 0));
/// ```

pub fn all_match<C: AsRef<[T]>, T>(collection: &C, predicate: impl Fn(&T, usize) -> bool) -> bool {
    let vec = collection.as_ref();

    for i in 0..vec.len() {
        if !predicate(&vec[i], i) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_match() {
        assert_eq!(true, all_match(&[1, 2, 3], &|n: &i32, _i: usize| *n <= 3));

        assert_eq!(
            false,
            all_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n % 2 == 0)
        );
    }
}
