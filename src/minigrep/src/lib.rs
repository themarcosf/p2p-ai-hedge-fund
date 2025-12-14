//! # MiniGrep Library
//!
//! This library provides functions to search for lines containing a specific query string
//! in a given text, both in case-sensitive and case-insensitive manners.

/// Searches for lines containing the query string in a case-sensitive manner.
///
/// ## Examples
///
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
///
/// Duct tape.";
///
/// let result = minigrep::search_case_sensitive(query, contents);
/// assert_eq!(result, vec!["safe, fast, productive."]);
/// ```
///
/// ## Panics
///
/// This function does not panic.
///
/// ## Errors
///
/// This function does not return errors.
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for lines containing the query string in a case-insensitive manner.
///
/// ## Examples
///
/// ```
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Trust me.";
///
/// let result = minigrep::search_case_insensitive(query, contents);
/// assert_eq!(result, vec!["Rust:", "Trust me."]);
/// ```
///
/// ## Panics
///
/// This function does not panic.
///
/// ## Errors
///
/// This function does not return errors.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_ascii_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            search_case_sensitive(query, contents),
            vec!["safe, fast, productive."]
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";

        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["Rust:", "Trust me."]
        )
    }
}
