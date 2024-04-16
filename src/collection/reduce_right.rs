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
/// use rufl::collection;
///
/// assert_eq!(9, collection::reduce_right(&vec!["hello", "rust"], |agg, item, _: usize| { agg + item.len() }, 0));
///
/// ```

pub fn reduce_right<C: AsRef<[T]>, T, U>(
    collection: &C,
    accumulator: impl Fn(&U, &T, usize) -> U,
    initial: U,
) -> U {
    let mut result = initial;

    let vector = collection.as_ref();
    for i in (0..vector.len()).rev() {
        result = accumulator(&result, &vector[i], i)
    }

    result
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_reduce_right() {
        assert_eq!(
            9,
            reduce_right(
                &vec!["hello", "rust"],
                |agg, item, _: usize| { agg + item.len() },
                0,
            )
        );

        // let empty_vec: Vec<i32> = vec![];
        // assert_eq!(
        //     vec![4, 5, 2, 3, 0, 1],
        //     reduce_right(
        //         &vec![vec![0, 1], vec![2, 3], vec![4, 5]],
        //         |agg: &Vec<i32>, item: &Vec<i32>, i: usize| { [agg, item].concat() },
        //         empty_vec
        //     )
        // );
    }
}
