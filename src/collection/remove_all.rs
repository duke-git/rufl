/// Remove all specific elements within the vector.
///
///
/// # Arguments
///
/// * `vec` - The vector to perform remove.
///
/// * `items` - Elements to be removed.
///
///
/// # Examples
///
/// ```
/// use rufl::collection;
///
/// let mut vec = vec!["a", "b", "c", "b"];
///
/// collection::remove_all(&mut vec, &"b");
///
/// assert_eq!(vec!["a", "c"], vec);
///
/// ```

pub fn remove_all<T: PartialEq>(vec: &mut Vec<T>, item: &T) {
    vec.retain(|val| *val != *item);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all() {
        let mut vec = vec!["a", "b", "c", "b"];

        remove_all(&mut vec, &"b");
        assert_eq!(vec!["a", "c",], vec);

        remove_all(&mut vec, &"b");
        assert_eq!(vec!["a", "c"], vec);
    }
}
