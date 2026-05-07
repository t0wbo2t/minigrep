//  In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents
//  (rather than the argument query).
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result:Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn case_ignore_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Note that query is now a String rather than a string slice because calling to_lowercase creates new data rather than referencing
    // existing data. When we pass query as an argument to the contains method now, we need to add an ampersand because the signature of
    // contains is defined to take a string slice.

    let query = query.to_lowercase();
    let mut result:Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "prod";
        let contents = "Rust:\nSafe, fast and, productive.\nAll three features packed in one language.";
        assert_eq!(vec!["Safe, fast and, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "Rust:\nSafe, fast and, productive.\nAll three features packed in one language\n[Rust Supremacy].";
        assert_eq!(vec!["Rust:", "[Rust Supremacy]."], case_ignore_search(query, contents));
    }
}
