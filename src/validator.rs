pub trait Validator {
    fn validate(&self) -> Result<(), String>;

    fn str_not_empty(&self, value: &str) -> bool {
        value.len() > 0
    }
}
