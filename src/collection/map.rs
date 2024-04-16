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
/// use rufl::collection;
///
/// assert_eq!(vec![2, 3, 4, 5, 6], collection::map(&vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n + 1 }));
///
/// assert_eq!(vec![1, 0, 1, 0, 1], collection::map(&vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n % 2 }));
///
/// ```

pub fn map<T, U>(vector: &Vec<T>, iteratee: impl Fn(&T, usize) -> U) -> Vec<U> {
    let mut result: Vec<U> = Vec::new();

    for (index, item) in vector.iter().enumerate() {
        result.push(iteratee(item, index));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            vec![2, 3, 4, 5, 6],
            map(&vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n + 1 })
        );

        assert_eq!(
            vec![1, 0, 1, 0, 1],
            map(&vec![1, 2, 3, 4, 5], &|n: &i32, _i: usize| { *n % 2 })
        );
    }
}
