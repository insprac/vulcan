pub fn vulcan_dir() -> String {
    env("VULCAN_DIR")
}

pub fn openai_api_key() -> String {
    env("OPENAI_API_KEY")
}

pub fn gemini_api_key() -> String {
    env("GEMINI_API_KEY")
}

fn env(name: &str) -> String {
    let value = std::env::var(name).unwrap_or_else(|_| panic!("Environment variable {} is required", name));
    if value.is_empty() {
        panic!("Environment variable {} is empty", name);
    }
    return value;
}
