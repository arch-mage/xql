pub fn join<I, E>(elems: I, sep: &str) -> String
where
    E: std::string::ToString,
    I: IntoIterator<Item = E>,
{
    elems
        .into_iter()
        .map(|elem| elem.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}

pub fn quote(s: &str, q: char) -> String {
    let mut quoted = s.replace(q, q.to_string().repeat(2).as_str());
    quoted.insert(0, q);
    quoted.push(q);
    quoted
}

pub fn quote_pretty(s: &str, q: char) -> String {
    let mut chars = s.chars();
    if chars.next().map(|c| !c.is_ascii_digit()).unwrap_or(true)
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return s.to_string();
    }
    quote(s, q)
}
