/// Returns the maximum value of a collection.
///
///
/// # Arguments
///
/// * `collection` - The collection to find maximum value.
///
///
/// # Returns
///
/// Returns the maximum value of a collection..
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(Some(3), collection::max(&vec![1, 2, 3, 2]));
///
/// let empty: Vec<i32> = vec![];
/// assert_eq!(None, collection::max(&empty));
///
/// ```

pub fn max<C: AsRef<[T]>, T: PartialOrd + Clone>(collection: &C) -> Option<T> {
    let vector =  collection.as_ref().to_vec();

    match vector.len() {
        0 => None,
        _ => {
            let mut max = &vector[0];

            for i in 1..vector.len() {
                let val = &vector[i];
                if *val > *max {
                    max = val;
                }
                // max = cmp::max(max, vector[i])
            }

            return Some(max.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(Some(3), max(&vec![1, 2, 3, 2]));

        let empty: Vec<i32> = vec![];
        assert_eq!(None, max(&empty));
    }
}
