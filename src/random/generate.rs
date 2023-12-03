/// Generate random string.
///
/// # Arguments
///
/// * `char_type` - The char type of random string.
/// * `length` - The size of random string.
///
/// # Returns
///
/// Returns random string.
///
/// # Examples
///

pub(super) fn generate_str(char_type: &str, length: usize) -> String {
    let mut chars = "";

    match char_type {
        "NUMBER" => chars = crate::random::NUMBERS,
        "UPPER" => chars = crate::random::UPPER_LETTERS,
        "LOWWER" => chars = crate::random::LOWWER_LETTERS,
        "LETTER" => chars = crate::random::LETTERS,
        "ALPHANUMERIC" => chars = crate::random::ALPHANUMERIC,
        "SYMBOL" => chars = crate::random::SYMBOL_CHARS,
        "ALL" => chars = crate::random::ALLCHARS,
        _ => {
            return String::from("");
        }
    }

    let chars = chars.as_bytes().to_vec();

    let random_chars = fastrand::choose_multiple(chars.iter(), length);
    let random_chars = random_chars.iter().map(|c| **c as u8).collect::<Vec<_>>();

    return String::from_utf8(random_chars).unwrap();
}
