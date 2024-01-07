/// Inserts an element at position `index` within the vector.
///
///
/// # Arguments
///
/// * `vec` - The vector to insert.
///
/// * `index` - The position of vector to insert at.
/// 
/// * `item` - The element to be inserted.
///
/// # Returns
///
/// Result true return if insert successed, error if if insert failed.
///
/// # Examples
///
/// ```
/// use ruf::collection;
/// 
/// let mut vec = vec!["a", "b", "c"];
/// 
/// let result = collection::insert_at(&mut vec, 3, "d");
/// 
/// assert_eq!(true, result.ok().unwrap());
/// assert_eq!(vec!["a", "b", "c", "d"], vec);
/// 
/// ```

pub fn insert_at<T:Default>(vec: &mut Vec<T>, index: usize, item: T) -> Result<bool, String> {
    if index > vec.len() {
        return Err(String::from("insert_at: index should be <= vecor length"));
    }

    vec.insert(index, item);  

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at() {
        let mut vec = vec![];
        
        let result = insert_at(&mut vec, 0, "a");
        assert_eq!(true, result.ok().unwrap());
        assert_eq!(vec!["a"], vec);

        let result = insert_at(&mut vec, 1, "b");
        assert_eq!(true, result.ok().unwrap());
        assert_eq!(vec!["a", "b"], vec);

        let result = insert_at(&mut vec, 3, "c");
        assert_eq!("insert_at: index should be <= vecor length", result.err().unwrap());
        assert_eq!(vec!["a", "b"], vec);
    }
}
