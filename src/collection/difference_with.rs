/// like `difference` except that it accepts comparator which is invoked to compare elements of collection to values.
/// The order and references of result values are determined by the first collection
///
/// * comparator function signature: ```fn(item1: T, item2: T) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to inspect.
///
/// * `compared_collection` - The values to compare exclude.
///
/// * `comparator` - The comparator invoked per element.
///
/// # Returns
///
/// Returns a new vector of filtered values.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(vec![1, 5], collection::difference_with(vec![1, 2, 3, 4, 5], vec![4, 5, 6, 7, 8], &|n1: i32, n2: i32| { n2 == n1 * 2}));
///
/// ```

pub fn difference_with<C: AsRef<[T]>, T: Copy + PartialEq>(
    collection: C,
    compared_collection: C,
    comparator: impl Fn(T, T) -> bool,
) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    fn get_index<U: Copy>(
        vector: &Vec<U>,
        item: U,
        comparison: impl Fn(U, U) -> bool,
    ) -> Option<usize> {
        for i in 0..vector.len() {
            let val = vector[i];

            if comparison(item, val) {
                return Some(i);
            }
        }
        None
    }

    let vector = collection.as_ref();
    let compared_vector = compared_collection.as_ref().to_vec();

    for i in 0..vector.len() {
        let item = vector[i];
        let index_op = get_index(&compared_vector, item, &comparator);

        match index_op {
            Some(_) => {}
            None => result.push(vector[i]),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_with() {
        fn is_double(n1: i32, n2: i32) -> bool {
            n2 == n1 * 2
        }

        assert_eq!(
            vec![1, 5],
            difference_with(vec![1, 2, 3, 4, 5], vec![4, 5, 6, 7, 8], &is_double)
        );
    }
}
