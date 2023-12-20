use config::{Config, File};
use std::collections::HashMap;

pub fn get_config() {
    // 初始化配置
    let settings = Config::builder()
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .add_source(File::with_name("config/base.toml"))
        .add_source(File::with_name("config/dev.toml"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "\n{:?} \n\n-----------",
        settings
            .try_deserialize::<HashMap<String, HashMap<String, String>>>()
            .unwrap()
    );
}
