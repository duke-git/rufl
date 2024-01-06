/// Returns true if there is no element of the collection pass the predicate function check.
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
/// Returns true if no element pass the predicate check, else false.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(true, collection::none_match(&[1, 2, 3], &|n: &i32, _i: usize| *n > 3));
///
/// assert_eq!(false, collection::none_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n % 2 == 0));
/// ```

pub fn none_match<C: AsRef<[T]>, T>(collection: &C, predicate: &dyn Fn(&T, usize) -> bool) -> bool {
    for i in 0..collection.as_ref().len() {
        let item = &collection.as_ref()[i];
        if predicate(item, i) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_match() {
        assert_eq!(true, none_match(&[1, 2, 3], &|n: &i32, _i: usize| *n > 3));

        assert_eq!(
            false,
            none_match(&vec![1, 2, 3], &|n: &i32, _i: usize| *n % 2 == 0)
        );
    }
}
