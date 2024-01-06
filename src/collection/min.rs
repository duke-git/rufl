/// Returns the minimum value of a collection.
///
///
/// # Arguments
///
/// * `collection` - The collection to find minimum value.
///
///
/// # Returns
///
/// The minimum value of a collection.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(Some(1), collection::min(&vec![1, 2, 3, 2]));
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(None, collection::min(&empty));
///
/// ```

pub fn min<C: AsRef<[T]>, T: PartialOrd + Clone>(collection: &C) -> Option<T> {
    let vector =  collection.as_ref().to_vec();

    match vector.len() {
        0 => None,
        _ => {
            let mut min = &vector[0];

            for i in 1..vector.len() {
                let val = &vector[i];
                if *val < *min {
                    min = val;
                }
            }

            return Some(min.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        assert_eq!(Some(1), min(&vec![1, 2, 3, 1]));

        let empty: Vec<i32> = vec![];
        assert_eq!(None, min(&empty));
    }
}
