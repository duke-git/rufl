/// Replace the first n `old`` elements with `new` elements within the vector.
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
/// * `n` - the number of old elements bo be replaced.
/// 
///
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// let mut vec = vec!["a", "b", "c", "b"];
///
/// let result = collection::replace_n(&mut vec, "b", "2", 1);
///
/// assert_eq!(vec!["a", "2", "c", "b"], vec);
///
/// ```

pub fn replace_n<T: PartialEq + Clone>(vec: &mut Vec<T>, old: T, new: T, mut n: usize) {
    for i in 0..vec.len() {
        if vec[i] == old && n !=0{
            vec[i] = new.clone();
            n -=1 ;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_n() {
        let mut vec = vec!["a", "b", "c", "b"];

        replace_n(&mut vec, "b", "2", 0);
        assert_eq!(vec!["a", "b", "c", "b"], vec);

        replace_n(&mut vec, "b", "2", 1);
        assert_eq!(vec!["a", "2", "c", "b"], vec);

        replace_n(&mut vec, "b", "2", 2);
        assert_eq!(vec!["a", "2", "c", "2"], vec);
    }
}
