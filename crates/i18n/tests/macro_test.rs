use hrs_server_macros::{i18n, t};

// i18n!("..");
i18n!();

#[test]
fn test_t() {
    t!();
    // println!("{}", t!());
    assert!(true);
}

#[test]
fn test_proc() {
    i18n_content();
    assert!(true);
}
