/// Creates new collection of element by running each element in collection thru iteratee.
///
/// * iteratee function signature: ```fn(item:T, index: usize) -> U```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The function invoked per iteration.
///
/// # Returns
///
/// Returns the new mapped collection.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!([2, 3, 4, 5, 6].to_vec(), collection::map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| { n + 1 }));
///
/// assert_eq!([1, 0, 1, 0, 1].to_vec(), collection::map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| n % 2 == 0));
/// ```

pub fn map<T: Copy, U: Copy>(collection: Vec<T>, iteratee: &dyn Fn(T, usize) -> U) -> Vec<U> {
    let mut result: Vec<U> = Vec::new();

    for i in 0..collection.len() {
        let val = collection[i];
        result.push(iteratee(val, i));
    }

    return result;
}

mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            [2, 3, 4, 5, 6].to_vec(),
            map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| { n + 1 })
        );

        assert_eq!(
            [1, 0, 1, 0, 1].to_vec(),
            map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| { n % 2 })
        );
    }
}
