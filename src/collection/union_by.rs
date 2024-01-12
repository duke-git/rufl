/// Creates a vector of unique elements between two collections.
/// it accepts iteratee which is invoked for each element of each collection to generate the criterion by which uniqueness is computed.
///
/// * iteratee function signature: ```fn(item1: &T) -> T```
///
/// # Arguments
///
/// * `collection1` - The collection to inspect.
///
/// * `collection2` - The collection to inspect.
///
/// * `iteratee` - The iteratee invoked per element.
///
/// # Returns
///
/// Returns the new vector of combined elements.
///
/// # Examples
///
/// ```
/// use ruf::collection;
///
/// assert_eq!(vec![1, 2, 3], collection::union_by([1, 2], [2, 3], &|a: &i32| { *a }));
///
/// assert_eq!(vec![2.1, 1.2], collection::union_by(vec![2.1, 2.2], vec![1.2, 1.3, 2.3], &|num: &f64| { num.floor() }));
///
/// ```
///
pub fn union_by<C: AsRef<[T]>, T: Clone + PartialEq>(
    collection1: C,
    collection2: C,
    iteratee: impl Fn(&T) -> T,
) -> Vec<T> {
    let merged_vector = [collection1.as_ref().to_vec(), collection2.as_ref().to_vec()].concat();

    let mut unique_vector = crate::collection::unique(merged_vector);

    let mut process_vector = Vec::new();

    unique_vector
        .iter()
        .for_each(|item| process_vector.push(iteratee(&item)));

    for i in (0..process_vector.len()).rev() {
        for j in (i + 1..process_vector.len()).rev() {
            if process_vector[i] == process_vector[j] {
                process_vector.remove(j);
                unique_vector.remove(j);
            }
        }
    }

    unique_vector
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_by() {
        assert_eq!(vec![1, 2, 3], union_by([1, 2], [2, 3], &|a: &i32| { *a }));
        assert_eq!(
            vec![2.1, 1.2],
            union_by(vec![2.1, 2.2], vec![1.2, 1.3, 2.3], &|num: &f64| {
                num.floor()
            })
        );
    }
}
