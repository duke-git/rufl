/// Replace all old items with new items within the vector.
///
///
/// # Arguments
///
/// * `vec` - The vector to replace.
///
/// * `old` - old item to be replaced.
///
/// * `new` - new item to replace.
///
/// # Returns
///
/// Returns the number of elements to be replaced.
///
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// let mut vec = vec!["a", "b", "c", "b"];
///
/// let result = collection::replace_all(&mut vec, "b", "2");
///
/// assert_eq!(vec!["a", "2", "c", "2"], vec);
///
/// ```

pub fn replace_all<T: PartialEq + Clone>(vec: &mut Vec<T>, old: T, new: T) -> usize {
    let mut repalce_count = 0;
    for i in 0..vec.len() {
        if vec[i] == old {
            vec[i] = new.clone();
            repalce_count += 1;
        }
    }

    repalce_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_all() {
        let mut vec = vec!["a", "b", "c", "b"];

        replace_all(&mut vec, "b", "2");
        assert_eq!(vec!["a", "2", "c", "2"], vec);

        replace_all(&mut vec, "d", "4");
        assert_eq!(vec!["a", "2", "c", "2"], vec);
    }
}
