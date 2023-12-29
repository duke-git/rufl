/// Creates a vector of unique elements between all collections.
///
/// # Arguments
///
/// * `collections` - The collections to inspect.
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
/// assert_eq!(vec![1, 2, 3, 4], collection::union(vec![[1, 2], [3, 4]]));
///
/// assert_eq!(vec![1, 2, 3, 4], collection::union(vec![[1, 2, 3], [2, 3, 4]]));
///
/// ```
pub fn union<C: AsRef<[T]>, T: Clone + PartialEq>(collections: Vec<C>) -> Vec<T> {
    if collections.len() == 0 {
        return Vec::new();
    }

    if collections.len() == 1 {
        return crate::collection::unique(&collections[0]);
    }

    fn reduce_union<U: Clone + PartialEq>(vector_a: &Vec<U>, vector_b: &Vec<U>) -> Vec<U> {
        let mut vector = vector_a.clone();

        for item in vector_b {
            vector.push(item.clone());
        }

        crate::collection::unique(vector)
    }

    let mut result = reduce_union(
        &collections[0].as_ref().to_vec(),
        &collections[1].as_ref().to_vec(),
    );

    let mut reduce_vectors = Vec::with_capacity(2);

    for i in 2..collections.len() {
        reduce_vectors.push(result);
        reduce_vectors.push(collections[i].as_ref().to_vec());

        result = reduce_union(&reduce_vectors[0], &reduce_vectors[1]);
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_union() {
        assert_eq!(vec![1, 2, 3, 4], union(vec![[1, 2], [3, 4]]));
        assert_eq!(vec![1, 2, 3, 4], union(vec![[1, 2, 3], [2, 3, 4]]));

        let empty_arr1: [i32; 0] = [];
        let empty_arr2: [i32; 0] = [];
        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(empty_vec, union(vec![empty_arr1, empty_arr2]));
    }
}
