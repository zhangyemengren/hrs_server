pub use hrs_server_proc_macro::i18n;

#[macro_export]
macro_rules! t {
    ($name_space:tt, $key:tt) => {{
        t!($name_space, $key, "en")
    }};
    ($name_space:tt, $key:tt, $lang:tt) => {{
        let j = I18N_MAP.get($name_space).unwrap().get($lang).unwrap();
        let s = serde_json::from_str::<serde_json::Value>(&j).unwrap();
        let r = s.get($key).unwrap().as_str().unwrap();
        r.to_string()
    }};
}
