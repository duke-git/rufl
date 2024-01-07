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

pub fn replace_all<T: PartialEq + Copy>(vec: &mut Vec<T>, old: T, new: T) {
    for i in 0..vec.len() {
        if vec[i] == old {
            // let _ = std::mem::replace(&mut vec[i], new);
            vec[i] = new;
        }
    }
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
