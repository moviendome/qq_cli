pub fn suggester(val: &str) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let commands = vec![
        //"install", "migrate", "console", "start", "test", "routes", "g", "gl", "gp", "gP", "gm", "ga", "exit", "help",
        "install", "start", "console", "test", "routes", "migrate",
    ];

    let suggestions = commands
        .into_iter()
        .filter(|cmd| cmd.starts_with(val))
        .map(|s| s.to_string())
        .collect();

    Ok(suggestions)
}
