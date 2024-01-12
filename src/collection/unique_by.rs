/// Calls a provided custom comparator with element of collection, returns a vector of unique element.
///
/// * comparator function signature: ```fn(item1: &T, item2: &T) -> bool```
///
/// # Arguments
///
/// * `collection` - The collection to inspect.
///
/// * `comparator` - The iteratee invoked per element.
///
/// # Returns
///
/// Returns a new duplicate free vector.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![1, 2, 3], collection::unique_by([1, 2, 2, 3, 3, 3], &|a: &i32, b: &i32| { a == b }));
///
/// assert_eq!(vec![1.0, 2.0, 3.0], collection::unique_by(vec![1.0, 1.1, 2.0, 2.1, 3.0, 3.1, 3.2], &|a: &f64, b: &f64| { a.floor() == b.floor()}));
///
/// ```
///
pub fn unique_by<C: AsRef<[T]>, T: Clone + PartialEq>(
    collection: C,
    comparator: impl Fn(&T, &T) -> bool,
) -> Vec<T> {
    let mut result = collection.as_ref().to_vec().clone();

    for i in (0..result.len()).rev() {
        for j in (i + 1..result.len()).rev() {
            if comparator(&result[i], &result[j]) {
                result.remove(j);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_by() {
        assert_eq!(
            vec![1, 2, 3],
            unique_by([1, 2, 3], &|a: &i32, b: &i32| { a == b })
        );
        assert_eq!(
            vec![1, 2, 3],
            unique_by([1, 2, 2, 3, 3, 3], &|a: &i32, b: &i32| { a == b })
        );

        assert_eq!(
            vec![1.0, 2.0, 3.0],
            unique_by(
                vec![1.0, 1.1, 2.0, 2.1, 3.0, 3.1, 3.2],
                &|a: &f64, b: &f64| { a.floor() == b.floor() }
            )
        );
    }
}
