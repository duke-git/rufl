/// Partition collection elements with the evaluation of the given predicate function.
///
/// * predicate function signature: ```fn(item: T, index: usize) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to perform partition.
///
/// * `predicate` - The function invoked per iteration.
///
/// # Returns
///
/// Returns two partitioned collections.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!((vec![2, 4], vec![1, 3, 5]), collection::partition(&vec![1, 2, 3, 4, 5], |n: &i32, _i: usize| n % 2 == 0));
///
/// ```

pub fn partition<T: Clone>(
    vector: &Vec<T>,
    predicate: impl Fn(&T, usize) -> bool,
) -> (Vec<T>, Vec<T>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for (index, item) in vector.iter().enumerate() {
        if predicate(item, index) {
            left.push(item.clone())
        } else {
            right.push(item.clone())
        }
    }

    return (left, right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(
            (vec![2, 4], vec![1, 3, 5]),
            partition(&vec![1, 2, 3, 4, 5], |n: &i32, _i: usize| n % 2 == 0)
        );
    }
}
