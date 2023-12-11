use hrs_server_macros::{i18n, t};

#[test]
fn test_t() {
    assert_eq!(t!(), "This is my macro!");
}
i18n!();

#[test]
fn test_proc() {
    assert_eq!(i18n(), 42);
}
