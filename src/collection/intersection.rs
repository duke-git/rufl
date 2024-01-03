/// Creates a vector of unique elements that included by the all collections.
///
/// # Arguments
///
/// * `collections` - The collection to inspect.
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
/// assert_eq!(vec![2, 3], collection::intersection(vec![vec![1, 2, 2, 3], vec![2, 3, 4]]));
///
///
/// ```
pub fn intersection<C: AsRef<[T]>, T: Clone + PartialEq>(collections: Vec<C>) -> Vec<T> {
    if collections.len() == 0 {
        return Vec::new();
    }

    if collections.len() == 1 {
        return crate::collection::unique(&collections[0]);
    }

    fn reduce_intersection<U: Clone + PartialEq>(vector_a: &Vec<U>, vector_b: &Vec<U>) -> Vec<U> {
        let mut vector = Vec::new();

        for item in vector_a {
            if vector_b.contains(item) {
                vector.push((*item).clone())
            }
        }

        crate::collection::unique(vector)
    }

    let mut result = reduce_intersection(
        &collections[0].as_ref().to_vec(),
        &collections[1].as_ref().to_vec(),
    );

    let mut reduce_vectors = Vec::with_capacity(2);

    for i in 2..collections.len() {
        reduce_vectors.push(result);
        reduce_vectors.push(collections[i].as_ref().to_vec());

        result = reduce_intersection(&reduce_vectors[0], &reduce_vectors[1]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let vec1 = vec![1, 2, 2, 3];
        let vec2 = vec![2, 3, 4];
        let vec3 = vec![0, 2, 3, 5, 6];
        let vec4 = vec![0, 4, 5];

        assert_eq!(vec![1, 2, 3], intersection(vec![&vec1]));
        assert_eq!(vec![2, 3], intersection(vec![&vec1, &vec2]));
        assert_eq!(vec![2, 3], intersection(vec![&vec1, &vec2, &vec3]));
        assert_eq!(vec![2, 3], intersection(vec![&vec1, &vec2, &vec3]));

        let empty_vec: Vec<i32> = Vec::new();
        assert_eq!(empty_vec, intersection(vec![&vec1, &vec4]));
    }
}
