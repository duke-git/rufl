/// Reduces collection to a value which is the accumulated result of running each element in collection thru iteratee,
/// where each successive invocation is supplied the return value of the previous.
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
/// assert_eq!(15, collection::reduce(&vec![1, 2, 3, 4, 5], |x: &i32, y: &i32, i: usize| { x + y }, 0));
///
/// assert_eq!(120, collection::reduce(&vec![1, 2, 3, 4, 5], |x: &i32, y: &i32, i: usize| { x * y }, 1));
///
/// ```

pub fn reduce<C: AsRef<[T]>, T, U>(
    collection: &C,
    accumulator: impl Fn(&U, &T, usize) -> U,
    initial: U,
) -> U {
    let mut result = initial;

    let vector = collection.as_ref();
    for (index, item) in vector.iter().enumerate() {
        result = accumulator(&result, item, index)
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce() {
        assert_eq!(
            15,
            reduce(
                &vec![1, 2, 3, 4, 5],
                |x: &i32, y: &i32, _: usize| { x + y },
                0
            )
        );

        assert_eq!(
            120,
            reduce(
                &vec![1, 2, 3, 4, 5],
                |x: &i32, y: &i32, _: usize| { x * y },
                1
            )
        );
    }
}
