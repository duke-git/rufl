/// Reduces collection to a value which is the accumulated result of running each element in collection thru iteratee,
/// where each successive invocation is supplied the return value of the previous.
///
/// * iteratee function signature: ```fn(item: T, item: T) -> T```
///
/// # Arguments
///
/// * `collection` - The collection to iterate over.
///
/// * `accumulator` - The initial value.
///
/// * `iteratee` - The function invoked per iteration.
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
/// assert_eq!(15, collection::reduce(vec![1, 2, 3, 4, 5], &0, &|x: i32, y: i32| { x + y }));
///
/// assert_eq!(120, collection::reduce(vec![1, 2, 3, 4, 5], &1, &|x: i32, y: i32| { x * y }));
///
/// ```

pub fn reduce<C: AsRef<[T]>, T: Copy>(
    collection: C,
    accumulator: &T,
    iteratee: &dyn Fn(T, T) -> T,
) -> T {
    let vector = collection.as_ref().to_vec();

    if vector.len() == 0 {
        return *accumulator;
    }

    let mut result = iteratee(*accumulator, vector[0]);

    let mut reduce_arr: [T; 2] = [*accumulator, *accumulator];

    for i in 1..vector.len() {
        reduce_arr[0] = result;
        reduce_arr[1] = vector[i];

        result = iteratee(reduce_arr[0], reduce_arr[1]);
    }

    result
}

mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            15,
            reduce(vec![1, 2, 3, 4, 5], &0, &|x: i32, y: i32| { x + y })
        );

        assert_eq!(
            120,
            reduce(vec![1, 2, 3, 4, 5], &1, &|x: i32, y: i32| { x * y })
        );
    }
}
