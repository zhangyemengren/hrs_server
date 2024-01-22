pub trait Validator {
    type Failure;
    fn validate(&self) -> Result<(), Self::Failure>;

    fn str_empty(&self, value: &str) -> bool {
        value.len() == 0
    }
}
