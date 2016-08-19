pub fn hello(name: Option<&str>) -> String {
    // This function accepts and returns a string.
    match name {
        // If name is not None, reformat the string.
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, World!".to_string()
    }
}
