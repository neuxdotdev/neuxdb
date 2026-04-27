pub fn split_quoted(s: &str, delim: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    for c in s.chars() {
        if c == '\'' && !in_quote {
            in_quote = true;
            current.push(c);
        } else if c == '\'' && in_quote {
            in_quote = false;
            current.push(c);
        } else if c == delim && !in_quote {
            result.push(current.trim().to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        result.push(current.trim().to_string());
    }
    result
}
