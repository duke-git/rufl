/// Iterates over elements of collection with predicate function, returns the number of all matched elements.
///
/// * predicate function signature: ```fn(item: &T, index: usize) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The check function invoked per iteration.
///
/// # Returns
///
/// Returns the number of all elements which pass the predicate check.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(2, collection::count_by([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n < 3));
///
/// assert_eq!(3, collection::count_by([1, 2, 3, 4, 5].to_vec(), &|n: &i32, _i: usize| *n % 2 != 0));
/// ```

pub fn count_by<C: AsRef<[T]>, T>(collection: C, predicate: impl Fn(&T, usize) -> bool) -> usize {
    let mut count = 0;
    let vec = collection.as_ref();

    for i in 0..vec.len() {
        if predicate(&vec[i], i) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_by() {
        assert_eq!(2, count_by([1, 2, 3, 4, 5], &|n: &i32, _i: usize| *n < 3));

        assert_eq!(
            3,
            count_by([1, 2, 3, 4, 5].to_vec(), &|n: &i32, _i: usize| *n % 2 != 0)
        );
    }
}
