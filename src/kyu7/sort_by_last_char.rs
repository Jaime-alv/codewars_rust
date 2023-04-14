fn sort_by_last_char(x: &str) -> Vec<String> {
    x.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
}