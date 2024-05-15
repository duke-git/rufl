pub(super) enum CharType {
    Numberic,
    LowerLetter,
    UpperLetter,
    Letter,
    AlphaNumberic,
    Symbol,
    All,
}

impl CharType {
    fn value(self) -> &'static str {
        match self {
            CharType::Numberic => "0123456789",
            CharType::LowerLetter => "abcdefghijklmnopqrstuvwxyz",
            CharType::UpperLetter => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            CharType::Letter => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            CharType::AlphaNumberic => {
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            }
            CharType::Symbol => "!@#$%^&*()_+-=[]{}|;':\",./<>?",
            CharType::All => "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+-=[]{}|;':\",./<>?",
        }
    }
}

pub(super) fn rand_string(char_type: CharType, length: usize) -> String {
    let chars = char_type.value();
    let chars = chars.as_bytes().to_vec();

    let random_chars = fastrand::choose_multiple(chars.iter(), length);
    let random_chars = random_chars.iter().map(|c| **c as u8).collect::<Vec<_>>();

    return String::from_utf8(random_chars).unwrap();
}
