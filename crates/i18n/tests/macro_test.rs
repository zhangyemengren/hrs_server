use hrs_server_macros::{i18n, t};

i18n!();

#[test]
fn test_t() {
    assert_eq!(t!("common", "hello"), "Hello World!");
    assert_eq!(t!("common", "hello", "cn"), "你好世界！");
    assert_eq!(t!("common", "hello", "jp"), "");
    assert_eq!(t!("common", "world", "en"), "");
    assert_eq!(t!("common", "world", "jp"), "");
    assert_eq!(t!("nonexistent", "hello"), "");
    assert_eq!(t!("nonexistent", "hello", "cn"), "");
    assert_eq!(t!("nonexistent", "hello", "jp"), "");
    assert_eq!(t!("nonexistent", "world", "en"), "");
    assert_eq!(t!("nonexistent", "world", "jp"), "");
}
