pub use hrs_server_proc_macro::i18n;

#[macro_export]
macro_rules! t {
    () => {{
        let w = "This is my macro!";
        println!("{}", w);
        w.to_string()
    }};
}