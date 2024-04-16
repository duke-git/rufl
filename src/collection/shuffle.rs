/// Returns a vector of shuffled values
///
/// # Arguments
///
/// * `collection` - The vector to shuffle.
///
/// # Returns
///
/// Returns the new shuffled vector.
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// assert_eq!(5, collection::shuffle(&vec![1, 2, 3, 4, 5]).len());
///
/// ```
pub fn shuffle<T: Copy>(collection: &Vec<T>) -> Vec<T> {
    let mut indexes: Vec<usize> = (0..collection.len()).collect();

    fastrand::shuffle(&mut indexes);

    let mut result = Vec::with_capacity(collection.len());

    for i in 0..indexes.len() {
        let rand_index = indexes[i];
        result.push(collection[rand_index]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle() {
        assert_eq!(5, shuffle(&vec![1, 2, 3, 4, 5]).len());
    }
}
