pub fn get_config_path() -> String {
    std::env::current_dir()
        .unwrap()
        .join("config.toml")
        .to_str()
        .unwrap()
        .to_string()
}
