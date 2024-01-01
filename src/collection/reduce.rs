/// Reduces collection to a value which is the accumulated result of running each element in collection thru iteratee,
/// where each successive invocation is supplied the return value of the previous.
///
/// * accumulator function signature: ```fn(item: T, item: T, index: usize) -> T```
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
/// assert_eq!(15, collection::reduce(vec![1, 2, 3, 4, 5], &|x: i32, y: i32, i: usize| { x + y }, &0));
///
/// assert_eq!(120, collection::reduce(vec![1, 2, 3, 4, 5], &|x: i32, y: i32, i: usize| { x * y }, &1));
///
/// ```

pub fn reduce<C: AsRef<[T]>, T: Copy>(
    collection: C,
    accumulator: &dyn Fn(T, T, usize) -> T,
    initial: &T,
) -> T {
    let vector = collection.as_ref().to_vec();

    let mut result = *initial;

    for i in 0..vector.len() {
        result = accumulator(result, vector[i], i)
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            15,
            reduce(
                vec![1, 2, 3, 4, 5],
                &|x: i32, y: i32, i: usize| { x + y },
                &0
            )
        );

        assert_eq!(
            120,
            reduce(
                vec![1, 2, 3, 4, 5],
                &|x: i32, y: i32, i: usize| { x * y },
                &1
            )
        );
    }
}
