/// like `difference` except that it accepts iteratee which is invoked for each element of collection and values to generate the criterion by which they're compared.
/// The order and references of result values are determined by the first collection
///
/// * iteratee function signature: ```fn(item: T, index: usize) -> T```
///
/// # Arguments
///
/// * `collection` - The collection to inspect.
///
/// * `compared_collection` - The values to compare exclude.
///
/// * `iteratee` - The iteratee invoked per element.
///
/// # Returns
///
/// Returns a new vector of filtered values.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![1.2], collection::difference_by([1.2, 2.1], [2.3, 3.4], &|n: &f64, _i: usize| {n.floor()}));
///
/// assert_eq!(vec![1, 2], collection::difference_by(vec![1, 2, 3, 4, 5], vec![3, 4, 5], &|n: &i32, _i: usize| { n + 1}));
/// ```

pub fn difference_by<C: AsRef<[T]>, T: Clone + PartialEq>(
    collection: C,
    compared_collection: C,
    iteratee: &dyn Fn(&T, usize) -> T,
) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();

    let c1 = crate::collection::map(collection.as_ref().to_vec(), iteratee);
    let c2 = crate::collection::map(compared_collection.as_ref().to_vec(), iteratee);

    let vector = collection.as_ref().to_vec();

    for i in 0..c1.len() {
        let item = &c1[i];

        if !c2.contains(&item) {
            result.push(vector[i].clone())
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_by() {
        fn floor(value: &f64, _: usize) -> f64 {
            value.floor()
        }
        assert_eq!(vec![1.2], difference_by([1.2, 2.1], [2.3, 3.4], &floor));

        assert_eq!(
            vec![1, 2],
            difference_by(vec![1, 2, 3, 4, 5], vec![3, 4, 5], &|n: &i32, _i: usize| {
                n + 1
            })
        );
    }
}
