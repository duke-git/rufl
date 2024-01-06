/// Creates new collection of element by running each element in collection thru iteratee.
///
/// * iteratee function signature: ```fn(item: &T, index: usize) -> U```
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
/// assert_eq!(vec![2, 3, 4, 5, 6], collection::map(vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n + 1 }));
///
/// assert_eq!(vec![1, 0, 1, 0, 1], collection::map(vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n % 2 }));
///
/// ```

pub fn map<T: Clone, U: Clone>(collection: Vec<T>, iteratee: &dyn Fn(&T, usize) -> U) -> Vec<U> {
    let mut result: Vec<U> = Vec::new();

    for i in 0..collection.len() {
        let item = &collection[i];
        result.push(iteratee(item, i));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            vec! [2, 3, 4, 5, 6],
            map(vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n + 1 })
        );

        assert_eq!(
            vec![1, 0, 1, 0, 1],
            map(vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n % 2 })
        );
    }
}
