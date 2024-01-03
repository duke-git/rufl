/// Return a collection which apply both filtering and mapping to the given collection.
///
/// * iteratee function should return a tuple within two elements.
/// 1. first element of the tuple is mapping result.
/// 2. second element the tuple tell whether the mapping result should be included in the returning collection or not.
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `predicate` - The function invoked per iteration.
///
/// # Returns
///
/// Returns the new filtered and mapped collection.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(
///     [4, 8].to_vec(),
///     collection::filter_map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| {
///         if n % 2 == 0 {
///             return (n * 2, true);
///         }
///         (0, false)
///     })
/// );
/// ```

pub fn filter_map<T: Copy, U: Copy>(
    collection: Vec<T>,
    iteratee: &dyn Fn(T, usize) -> (U, bool),
) -> Vec<U> {
    let mut result: Vec<U> = Vec::new();

    for i in 0..collection.len() {
        let item = iteratee(collection[i], i);
        if item.1 {
            result.push(item.0);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            [4, 8].to_vec(),
            filter_map([1, 2, 3, 4, 5].to_vec(), &|n: i32, _i: usize| {
                if n % 2 == 0 {
                    return (n * 2, true);
                }

                (0, false)
            })
        );
    }
}
