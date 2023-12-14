use hrs_server_macros::{i18n, t};

#[test]
fn test_t() {
    assert_eq!(t!(), "This is my macro!");
}
i18n!("..");
// i18n!();
#[test]
fn test_proc() {
    i18n_content();
    assert!(true);
}
