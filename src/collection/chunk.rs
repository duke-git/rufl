/// Returns a vector of elements split into groups the length of size. If vector can't be split evenly,
//  the final chunk will be the remaining elements.
///
/// # Arguments
///
/// * `vec` - The collection to perform chunk.
///
/// * `size` - The function invoked per iteration.
///
/// # Returns
///
/// Returns the new chunked vector.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(vec![vec![1, 2], vec![3, 4], vec![5]], collection::chunk(vec![1, 2, 3, 4, 5], 2));
///
/// assert_eq!(vec![vec![1, 2, 3], vec![4, 5]], collection::chunk(vec![1, 2, 3, 4, 5], 3));
///
/// ```

pub fn chunk<T: Clone>(vec: Vec<T>, size: usize) -> Vec<Vec<T>> {
    vec.chunks(size)
        .map(|item| item.to_vec())
        .collect::<Vec<Vec<T>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        assert_eq!(
            vec![vec![1], vec![2], vec![3], vec![4], vec![5]],
            chunk(vec![1, 2, 3, 4, 5], 1)
        );

        assert_eq!(
            vec![vec![1, 2], vec![3, 4], vec![5]],
            chunk(vec![1, 2, 3, 4, 5], 2)
        );

        assert_eq!(
            vec![vec![1, 2, 3], vec![4, 5]],
            chunk(vec![1, 2, 3, 4, 5], 3)
        );

        assert_eq!(
            vec![vec![1, 2, 3, 4], vec![5]],
            chunk(vec![1, 2, 3, 4, 5], 4)
        );

        assert_eq!(vec![vec![1, 2, 3, 4, 5]], chunk(vec![1, 2, 3, 4, 5], 5));
        assert_eq!(vec![vec![1, 2, 3, 4, 5]], chunk(vec![1, 2, 3, 4, 5], 6));
    }
}
