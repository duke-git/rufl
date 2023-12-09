/// Iterates over elements of collection, returning a collection of all elements pass the predicate function.
///
/// * predicate function signature: ```fn(item:T, index: usize) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The function invoked per iteration.
///
/// # Returns
///
/// Returns the new filtered collection.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!([1, 2, 3].to_vec(),collection::filter([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| n <= 3));
///
/// assert_eq!([2, 4].to_vec(),collection::filter([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| n % 2 == 0));
/// ```

pub fn filter<T: Copy>(collection: Vec<T>, predicate: &dyn Fn(T, usize) -> bool) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    for i in 0..collection.len() {
        let val = collection[i];
        if predicate(val, i) {
            result.push(val);
        }
    }

    return result;
}

mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            [1, 2, 3].to_vec(),
            filter([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| n <= 3)
        );

        assert_eq!(
            [2, 4].to_vec(),
            filter([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| n % 2 == 0)
        );
    }
}
