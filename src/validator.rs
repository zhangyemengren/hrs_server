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
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
                .unwrap()
        });
        RE.is_match(value)
    }
}
