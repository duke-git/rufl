/// Returns true if any element of the collection pass the predicate function check.
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
/// Returns true if any element pass the predicate check, else false.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(true, collection::some_match(&[1, 4, 5], &|n: &i32, _i: usize| *n <= 3));
///
/// assert_eq!(false, collection::some_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n > 3));
/// ```

pub fn some_match<C: AsRef<[T]>, T>(collection: &C, predicate: &dyn Fn(&T, usize) -> bool) -> bool {
    let vec = collection.as_ref();

    for i in 0..vec.len() {
        if predicate(&vec[i], i) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_match() {
        assert_eq!(true, some_match(&[1, 4, 5], &|n: &i32, _i: usize| *n <= 3));

        assert_eq!(
            false,
            some_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n > 3)
        );
    }
}
