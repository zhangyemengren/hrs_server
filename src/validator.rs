use once_cell::sync::Lazy;
use regex_lite::Regex;

pub trait Validator {
    type Failure;
    fn validate(&self) -> Result<(), Self::Failure>;

    fn str_empty(&self, value: &str) -> bool {
        value.len() == 0
    }

    fn username_reg_validate(&self, value: &str) -> bool {
        if value == "admin" {
            return true;
        }
        // 优化 https://docs.rs/regex/latest/regex/#avoid-re-compiling-regexes-especially-in-a-loop
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap());
        RE.is_match(value)
    }
    fn password_reg_validate(&self, value: &str) -> bool {
        if value == "admin" {
            return true;
        }
        // regex 不支持 前瞻（look-around）r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$"
        let has_lowercase = value.chars().any(|c| c.is_lowercase());
        let has_uppercase = value.chars().any(|c| c.is_uppercase());
        let has_digit = value.chars().any(|c| c.is_digit(10));
        let has_special_char = value.chars().any(|c| "@$!%*?&".contains(c));
        let is_at_least_8_chars = value.len() >= 8;

        has_lowercase && has_uppercase && has_digit && has_special_char && is_at_least_8_chars
    }
}
