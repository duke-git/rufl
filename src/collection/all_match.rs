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
/// assert_eq!(true, collection::all_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n <= 3));
///
/// assert_eq!(false, collection::all_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n % 2 == 0));
/// ```

pub fn all_match<C: AsRef<[T]>, T>(collection: C, predicate: &dyn Fn(&T, usize) -> bool) -> bool {
    for i in 0..collection.as_ref().len() {
        let val = &collection.as_ref()[i];
        if !predicate(val, i) {
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
        assert_eq!(
            true,
            all_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n <= 3)
        );

        assert_eq!(
            false,
            all_match([1, 2, 3].to_vec(), &|n: &i32, _i: usize| *n % 2 == 0)
        );
    }
}
