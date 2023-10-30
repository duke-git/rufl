/// Cut searches for the substring 'sep' in the source string, and splits the source string into two parts at the first occurrence of the substring 'sep': before and after.
/// returns a tuple (before:String, after:String, found:bool)
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo1 = string::cut("rust", "ru");
/// assert_eq!(":st:true", format!("{}:{}:{}", foo1.0, foo1.1, foo1.2));
///
/// let foo2 = string::cut("rust", "us");
/// assert_eq!("r:t:true", format!("{}:{}:{}", foo2.0, foo2.1, foo2.2));
///
/// let bar1 = string::cut("rust", "st");
/// assert_eq!("ru::true", format!("{}:{}:{}", bar1.0, bar1.1, bar1.2));
///
/// let bar2 = string::cut("rust", "abc");
/// assert_eq!("rust::false", format!("{}:{}:{}", bar2.0, bar2.1, bar2.2));
///
/// ```

pub fn cut(s: impl AsRef<str>, sep: &str) -> (String, String, bool) {
    match s.as_ref().find(sep) {
        Some(index) => (
            s.as_ref()[..index].to_string(),
            s.as_ref()[index + sep.len()..].to_string(),
            true,
        ),
        None => (s.as_ref().to_string(), "".to_string(), false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut() {
        let foo1 = cut("rust", "ru");
        assert_eq!(":st:true", format!("{}:{}:{}", foo1.0, foo1.1, foo1.2));

        let foo2 = cut("rust", "us");
        assert_eq!("r:t:true", format!("{}:{}:{}", foo2.0, foo2.1, foo2.2));

        let bar1 = cut("rust", "st");
        assert_eq!("ru::true", format!("{}:{}:{}", bar1.0, bar1.1, bar1.2));

        let bar2 = cut("rust", "abc");
        assert_eq!("rust::false", format!("{}:{}:{}", bar2.0, bar2.1, bar2.2));
    }
}
