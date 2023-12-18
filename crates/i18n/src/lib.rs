pub use hrs_server_proc_macro::i18n;

#[macro_export]
macro_rules! t {
    ($name_space:tt, $key:tt) => {{
        t!($name_space, $key, "en")
    }};
    ($name_space:tt, $key:tt, $lang:tt) => {{
        match I18N_MAP.get($name_space) {
            None => {
                println!("Name Space:{} Not Found", $name_space);
                "".to_string()
            }
            Some(ns) => match ns.get($lang) {
                None => {
                    println!("Language Key:{} Not Found", $lang);
                    "".to_string()
                }
                Some(lang_str) => match serde_json::from_str::<serde_json::Value>(lang_str) {
                    Err(_) => {
                        println!(
                            "Json Parse Error on Name Space:{} Language Key:{} Key:{}",
                            $name_space, $lang, $key
                        );
                        "".to_string()
                    }
                    Ok(json) => match json.get($key) {
                        None => {
                            println!("Key:{} Not Found", $key);
                            "".to_string()
                        }
                        Some(v) => v
                            .as_str()
                            .unwrap_or_else(|| {
                                println!("Key:{} Not String", $key);
                                ""
                            })
                            .to_string(),
                    },
                },
            },
        }
    }};
}
