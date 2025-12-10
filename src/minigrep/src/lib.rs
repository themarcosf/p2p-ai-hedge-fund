pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut out = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            out.push(line);
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_returns_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(search(query, contents), vec!["safe, fast, productive."])
    }
}
