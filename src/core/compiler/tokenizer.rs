pub(super) fn tokenize(sql: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = sql.chars().peekable();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '\'' {
            chars.next();
            let mut s = String::new();
            s.push('\'');
            while let Some(&c) = chars.peek() {
                s.push(c);
                chars.next();
                if c == '\'' {
                    break;
                }
            }
            tokens.push(s);
            continue;
        }
        if ch == '!' || ch == '<' || ch == '>' || ch == '=' {
            let mut op = String::new();
            while let Some(&c) = chars.peek() {
                if c == '!' || c == '<' || c == '>' || c == '=' {
                    op.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(op);
            continue;
        }
        if "(),*|;".contains(ch) {
            tokens.push(ch.to_string());
            chars.next();
            continue;
        }
        let mut word = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                word.push(c);
                chars.next();
            } else {
                break;
            }
        }
        if !word.is_empty() {
            tokens.push(word);
        }
    }
    tokens
}
