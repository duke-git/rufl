/// Reduce right like reduce except that it iterates over elements of collection from right to left.
///
/// * accumulator function signature: ```fn(agg: U, item: T, index: usize) -> U```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `accumulator` - The function invoked per iteration.
///
/// * `initial` - The initial value.
///
///
/// # Returns
///
/// Returns the accumulated value.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![4, 5, 2, 3, 0, 1], collection::reduce_right(vec![vec![0, 1], vec![2, 3], vec![4, 5]], &|agg: Vec<i32>, item: Vec<i32>, i: usize| { [agg, item].concat() }, &Vec::new()));
///
/// ```

pub fn reduce_right<C: AsRef<[T]>, T: Clone, U: Clone>(
    collection: C,
    accumulator: &dyn Fn(U, T, usize) -> U,
    initial: &U,
) -> U {
    let vector = collection.as_ref().to_vec();

    let mut result = (*initial).clone();

    for i in (0..vector.len()).rev() {
        result = accumulator(result.clone(), vector[i].clone(), i)
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_reduce_right() {
        assert_eq!(
            vec![4, 5, 2, 3, 0, 1],
            reduce_right(
                vec![vec![0, 1], vec![2, 3], vec![4, 5]],
                &|agg: Vec<i32>, item: Vec<i32>, i: usize| { [agg, item].concat() },
                &Vec::new()
            )
        );
    }
}
