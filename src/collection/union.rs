/// Creates a vector of unique elements between two collections.
///
/// # Arguments
///
/// * `collection1` - The collection to inspect.
///
/// * `collection2` - The collection to inspect.
///
/// # Returns
///
/// Returns the new vector of combined elements.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![1, 2, 3, 4], collection::union([1, 2], [3, 4]));
///
/// assert_eq!(vec![1, 2, 3, 4], collection::union([1, 2, 3], [2, 3, 4]));
///
/// ```
pub fn union<C: AsRef<[T]>, T: Clone + PartialEq>(collection1: C, collection2: C) -> Vec<T> {
    let vector1 = collection1.as_ref().to_vec();
    let vector2 = collection2.as_ref().to_vec();

    let mut result = vector1;

    for item in vector2 {
        result.push(item);
    }

    crate::collection::unique(result)
}

mod tests {
    use super::*;

    #[test]
    fn test_union() {
        assert_eq!(vec![1, 2, 3, 4], union([1, 2], [3, 4]));
        assert_eq!(vec![1, 2, 3, 4], union([1, 2, 3], [2, 3, 4]));

        let empty_arr1: [i32; 0] = [];
        let empty_arr2: [i32; 0] = [];
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(empty_vec, union(empty_arr1, empty_arr2));
    }
}
