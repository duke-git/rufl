/// Creates a vector of values not included in the other given collections using equality comparisons.
/// The order and references of result values are determined by the first collection
///
/// # Arguments
///
/// * `collection` - The collection to inspect.
///
/// * `compared_collection` - The values to compare exclude.
///
///
/// # Returns
///
/// Returns a new collection of filtered values.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![1], collection::difference([1, 2], [2, 3]));
///
/// assert_eq!(vec!["b"], collection::difference(["a", "b"], ["a", "c"]));
/// ```

pub fn difference<C: AsRef<[T]>, T: Copy + PartialEq>(
    collection: C,
    compared_collection: C,
) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    let compared_vector = compared_collection.as_ref().to_vec();

    collection.as_ref().into_iter().for_each(|item| {
        if !compared_vector.contains(item) {
            result.push(*item)
        }
    });

    return result;
}

mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        assert_eq!(vec![1], difference([1, 2], [2, 3]));
        assert_eq!(vec![1, 4], difference(vec![1, 2, 4], vec![2, 3]));
        assert_eq!(vec!["b"], difference(["a", "b"], ["a", "c"]));
    }
}
