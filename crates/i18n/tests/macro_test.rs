use hrs_server_macros::{i18n, t};

// i18n!("..");
i18n!();

#[test]
fn test_t() {
    assert_eq!(t!("common", "hello"), "Hello World!");
    assert_eq!(t!("common", "hello", "cn"), "你好世界！");
}

