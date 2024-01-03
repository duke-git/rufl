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
/// assert_eq!(true, collection::some_match([1, 4, 5].to_vec(), &|n: &i32, _i: usize| *n <= 3));
///
/// assert_eq!(false, collection::some_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n > 3));
/// ```

pub fn some_match<C: AsRef<[T]>, T>(collection: C, predicate: &dyn Fn(&T, usize) -> bool) -> bool {
    for i in 0..collection.as_ref().len() {
        let val = &collection.as_ref()[i];
        if predicate(val, i) {
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
        assert_eq!(
            true,
            some_match([1, 4, 5].to_vec(), &|n: &i32, _i: usize| *n <= 3)
        );

        assert_eq!(
            false,
            some_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n > 3)
        );
    }
}
