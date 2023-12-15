pub use hrs_server_proc_macro::i18n;

#[macro_export]
macro_rules! t {
    () => {{
        let j = I18N_MAP.get("common").unwrap().get("cn").unwrap();
        let s = serde_json::from_str::<serde_json::Value>(&j).unwrap();
        let r = s.get("hello").unwrap().as_str().unwrap();
        println!("{}", r);
        r
    }};
}
